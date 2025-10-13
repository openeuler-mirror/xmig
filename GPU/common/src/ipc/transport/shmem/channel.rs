// SPDX-License-Identifier: Mulan PSL v2
/*
 * Copyright (c) 2025 Huawei Technologies Co., Ltd.
 * This software is licensed under Mulan PSL v2.
 * You can use this software according to the terms and conditions of the Mulan PSL v2.
 * You may obtain a copy of Mulan PSL v2 at:
 *         http://license.coscl.org.cn/MulanPSL2
 *
 * THIS SOFTWARE IS PROVIDED ON AN "AS IS" BASIS, WITHOUT WARRANTIES OF ANY KIND,
 * EITHER EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO NON-INFRINGEMENT,
 * MERCHANTABILITY OR FIT FOR A PARTICULAR PURPOSE.
 * See the Mulan PSL v2 for more details.
 */

use std::{
    fmt::{Debug, Display},
    io,
    ops::Deref,
    ptr,
    sync::atomic::{AtomicU8, AtomicUsize, Ordering},
    thread,
    time::{Duration, Instant},
};

use linux_futex::{Futex, Shared};
use tracing::debug;

use crate::sys::{cache::CacheLineAligned, futex::FutexMutex};

use super::{
    buffer::{ShmemReadBuffer, ShmemWriteBuffer},
    error::ShmemTransportError,
    memory::ShmemRegion,
};

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum ShmemChannelState {
    #[default]
    Uninited = 0,
    Ready = 1,
    Closed = 2,
}

impl TryFrom<u8> for ShmemChannelState {
    type Error = ShmemTransportError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShmemChannelState::Uninited),
            1 => Ok(ShmemChannelState::Ready),
            2 => Ok(ShmemChannelState::Closed),
            _ => Err(ShmemTransportError::InvalidConnectionState),
        }
    }
}

impl Display for ShmemChannelState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self, f)
    }
}

/// A control block located at the start of the shmem for synchronization.
#[repr(C)]
#[derive(Debug)]
pub struct ShmemCtrlBlock {
    /// Read cursor. Incremented by readers.
    pub head: CacheLineAligned<AtomicUsize>,
    /// Write cursor. Incremented by writers.
    pub tail: CacheLineAligned<AtomicUsize>,
    /// State of the channel.
    state: CacheLineAligned<AtomicU8>,
    /// Mutex to ensure exclusive access for buffer.
    buf_lock: CacheLineAligned<FutexMutex>,
    /// Futex for readers to wait on when the buffer is empty.
    readable: CacheLineAligned<Futex<Shared>>,
    /// Futex for writers to wait on when the buffer is full.
    writable: CacheLineAligned<Futex<Shared>>,
}

impl ShmemCtrlBlock {
    #[inline]
    pub fn get_state(&self) -> Result<ShmemChannelState, ShmemTransportError> {
        ShmemChannelState::try_from(self.state.load(Ordering::Acquire))
    }

    #[inline]
    pub fn set_state(&self, state: ShmemChannelState) {
        self.state.store(state as u8, Ordering::Release);
    }

    #[inline]
    fn wait_readable(&self, last_value: u32) {
        let _ = self.readable.wait(last_value);
    }

    #[inline]
    pub fn notify_readable(&self) {
        self.readable.value.fetch_add(1, Ordering::Relaxed);
        self.readable.wake(1);
    }

    #[inline]
    pub fn notify_all_readable(&self) {
        self.readable.value.fetch_add(1, Ordering::Relaxed);
        self.readable.wake(i32::MAX);
    }

    #[inline]
    fn wait_writable(&self, last_value: u32) {
        let _ = self.writable.wait(last_value);
    }

    #[inline]
    pub fn notify_writable(&self) {
        self.writable.value.fetch_add(1, Ordering::Relaxed);
        self.writable.wake(1);
    }

    #[inline]
    pub fn notify_all_writable(&self) {
        self.writable.value.fetch_add(1, Ordering::Relaxed);
        self.writable.wake(i32::MAX);
    }
}

/// Represents a single direction of communication over a shared memory segment.
#[derive(Debug)]
pub struct ShmemChannel {
    memory: ShmemRegion,
    control_ptr: *mut ShmemCtrlBlock,
    buffer_ptr: *mut u8,
}

// SAFETY: All access to raw pointers is synchronized by atomic operations and FutexMutexes.
unsafe impl Send for ShmemChannel {}
unsafe impl Sync for ShmemChannel {}

impl Deref for ShmemChannel {
    type Target = ShmemCtrlBlock;
    fn deref(&self) -> &Self::Target {
        // SAFETY: The control pointer is valid for the lifetime of Channel.
        unsafe { &*self.control_ptr }
    }
}

impl ShmemChannel {
    pub fn create<S: AsRef<str>>(name: S, buffer_size: usize) -> Result<Self, ShmemTransportError> {
        let reserve_size = size_of::<ShmemCtrlBlock>();

        let memory = ShmemRegion::create(&name, buffer_size, reserve_size).map_err(|e| {
            ShmemTransportError::CreationError {
                name: name.as_ref().to_owned(),
                source: e,
            }
        })?;
        let control_ptr = memory.reserved_ptr() as *mut ShmemCtrlBlock;
        let buffer_ptr = memory.data_ptr();

        let channel = Self {
            memory,
            control_ptr,
            buffer_ptr,
        };

        // SAFETY: The shared memory is newly created and correctly sized.
        // As the owner, we can safely initialize the control block.
        unsafe {
            ptr::write(
                channel.control_ptr,
                ShmemCtrlBlock {
                    head: CacheLineAligned(AtomicUsize::new(0)),
                    tail: CacheLineAligned(AtomicUsize::new(0)),
                    state: CacheLineAligned(AtomicU8::new(ShmemChannelState::Uninited as u8)),
                    buf_lock: CacheLineAligned(FutexMutex::new()),
                    readable: CacheLineAligned(Futex::new(0)),
                    writable: CacheLineAligned(Futex::new(0)),
                },
            );
        }
        channel.set_state(ShmemChannelState::Ready);
        debug!("[Shmem] '{}': Ready", channel);

        Ok(channel)
    }

    pub fn open<S: AsRef<str>>(name: S, timeout: Duration) -> Result<Self, ShmemTransportError> {
        const RETRY_DELAY: Duration = Duration::from_millis(10);

        let start_time = Instant::now();
        let shmem = loop {
            if start_time.elapsed() >= timeout {
                return Err(ShmemTransportError::ConnectionTimeout);
            }
            match ShmemRegion::open(&name, size_of::<ShmemCtrlBlock>()) {
                Ok(shmem) => break shmem,
                Err(e) if e.kind() == io::ErrorKind::NotFound => {
                    thread::sleep(RETRY_DELAY);
                    continue;
                }
                Err(e) => {
                    return Err(ShmemTransportError::OpenError {
                        name: name.as_ref().to_owned(),
                        source: e,
                    });
                }
            }
        };

        let control_ptr = shmem.reserved_ptr() as *mut ShmemCtrlBlock;
        let buffer_ptr = shmem.data_ptr();

        let channel = Self {
            memory: shmem,
            control_ptr,
            buffer_ptr,
        };

        loop {
            if start_time.elapsed() >= timeout {
                return Err(ShmemTransportError::ConnectionTimeout);
            }
            match channel.get_state() {
                Ok(ShmemChannelState::Ready) => break,
                Ok(ShmemChannelState::Closed) => return Err(ShmemTransportError::ConnectionClosed),
                _ => thread::sleep(RETRY_DELAY),
            }
        }

        debug!("[Shmem] '{}': Ready", channel);
        Ok(channel)
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.memory.name()
    }

    #[inline]
    pub fn is_owner(&self) -> bool {
        self.memory.is_owner()
    }

    #[inline]
    pub fn capacity(&self) -> usize {
        self.memory.data_len()
    }

    pub fn read_buf(&'_ self) -> Result<ShmemReadBuffer<'_>, ShmemTransportError> {
        loop {
            if self.get_state()? == ShmemChannelState::Closed {
                return Err(ShmemTransportError::ConnectionClosed);
            }

            let guard = self.buf_lock.lock();

            let head = self.head.load(Ordering::Relaxed);
            let tail = self.tail.load(Ordering::Acquire);
            let readable_bytes = tail.wrapping_sub(head);

            if readable_bytes > 0 {
                let offset = head % self.capacity();
                let ptr = unsafe { self.buffer_ptr.add(offset) };

                // We can treat all `readable_bytes` as a single, contiguous slice.
                // The virtual memory mirroring handles any "wrap-around" seamlessly.
                debug!("[Shmem] '{}': Reading...", self);
                return Ok(ShmemReadBuffer::new(guard, self, ptr, readable_bytes));
            }

            // Before releasing the lock, read the current value of the futex event counter.
            let last_value = self.readable.value.load(Ordering::Relaxed);
            drop(guard);

            debug!("[Shmem] '{}': Waiting readable...", self);
            self.wait_readable(last_value);
        }
    }

    pub fn write_buf(&'_ self) -> Result<ShmemWriteBuffer<'_>, ShmemTransportError> {
        loop {
            if self.get_state()? == ShmemChannelState::Closed {
                return Err(ShmemTransportError::ConnectionClosed);
            }

            let guard = self.buf_lock.lock();

            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Relaxed);
            let used_space = tail.wrapping_sub(head);

            let writable_bytes = self.capacity() - used_space - 1;
            if writable_bytes > 0 {
                let offset = tail % self.capacity();
                let ptr = unsafe { self.buffer_ptr.add(offset) };

                // We can offer the entire `writable_bytes` as a single, contiguous slice.
                // The virtual memory mirroring handles any "wrap-around" seamlessly.
                debug!("[Shmem] '{}': Writting...", self);
                return Ok(ShmemWriteBuffer::new(guard, self, ptr, writable_bytes));
            }

            // Before releasing the lock, read the current value of the futex event counter.
            let last_value = self.writable.value.load(Ordering::Relaxed);
            drop(guard);

            debug!("[Shmem] '{}': Waiting writable...", self);
            self.wait_writable(last_value);
        }
    }

    pub fn close(&self) {
        self.set_state(ShmemChannelState::Closed);
        self.notify_all_readable();
        self.notify_all_writable();
    }
}

impl Drop for ShmemChannel {
    fn drop(&mut self) {
        if self.is_owner() {
            self.close();
        }
    }
}

impl Display for ShmemChannel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.name(), f)
    }
}
