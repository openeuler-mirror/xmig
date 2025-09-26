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
    ops::{Deref, DerefMut},
    sync::atomic::Ordering,
};

use crate::{
    ipc::transport::{ReadBuf, TransportError, WriteBuf},
    sys::futex::FutexMutexGuard,
};

use super::channel::ShmemChannel;

#[derive(Debug)]
pub struct ShmemReadBuffer<'a> {
    _guard: FutexMutexGuard<'a>,
    channel: &'a ShmemChannel,
    ptr: *mut u8,
    len: usize,
    consumed: usize,
}

impl<'a> ShmemReadBuffer<'a> {
    /// Creates a new read buffer.
    #[inline]
    pub fn new(
        guard: FutexMutexGuard<'a>,
        channel: &'a ShmemChannel,
        ptr: *mut u8,
        len: usize,
    ) -> Self {
        Self {
            _guard: guard,
            channel,
            ptr,
            len,
            consumed: 0,
        }
    }
}

impl ReadBuf for ShmemReadBuffer<'_> {
    /// Consumes the specified number of bytes from the buffer, advancing the
    /// `head` pointer and notifying waiting writers.
    fn consume(mut self, bytes: usize) -> Result<(), TransportError> {
        if bytes > self.len {
            return Err(TransportError::ReadBufferOverflow {
                consume: bytes,
                capacity: self.len,
            });
        }

        self.consumed = bytes;
        Ok(())
    }
}

impl<'a> Deref for ShmemReadBuffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        // SAFETY: The existence of the lock guard (`_guard`) guarantees that
        // we have exclusive read access and that the pointer and length are valid
        // for the lifetime of this struct.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a> DerefMut for ShmemReadBuffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The existence of the lock guard (`_guard`) guarantees that
        // we have exclusive read access and that the pointer and length are valid
        // for the lifetime of this struct. A read buffer is typically read-only,
        // but providing `DerefMut` can be useful in some protocols.
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for ShmemReadBuffer<'_> {
    fn drop(&mut self) {
        if self.consumed > 0 {
            self.channel
                .head
                .fetch_add(self.consumed, Ordering::Release);
            self.channel.notify_writable();
        }
    }
}

#[derive(Debug)]
pub struct ShmemWriteBuffer<'a> {
    _guard: FutexMutexGuard<'a>,
    channel: &'a ShmemChannel,
    ptr: *mut u8,
    len: usize,
    submitted: usize,
}

impl<'a> ShmemWriteBuffer<'a> {
    /// Creates a new write buffer.
    #[inline]
    pub fn new(
        guard: FutexMutexGuard<'a>,
        channel: &'a ShmemChannel,
        ptr: *mut u8,
        len: usize,
    ) -> Self {
        Self {
            _guard: guard,
            channel,
            ptr,
            len,
            submitted: 0,
        }
    }
}

impl WriteBuf for ShmemWriteBuffer<'_> {
    /// Submits the specified number of bytes to the buffer, advancing the
    /// `tail` pointer and notifying waiting readers.
    fn submit(mut self, bytes: usize) -> Result<(), TransportError> {
        if bytes > self.len {
            return Err(TransportError::WriteBufferOverflow {
                submit: bytes,
                capacity: self.len,
            });
        }

        self.submitted = bytes;
        Ok(())
    }
}

impl<'a> Deref for ShmemWriteBuffer<'a> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        // SAFETY: The existence of the lock guard (`_guard`) guarantees that
        // we have exclusive access and that the pointer and length are valid
        // for the lifetime of this struct.
        unsafe { std::slice::from_raw_parts(self.ptr, self.len) }
    }
}

impl<'a> DerefMut for ShmemWriteBuffer<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY: The existence of the lock guard (`_guard`) guarantees that
        // we have exclusive access and that the pointer and length are valid
        // for the lifetime of this struct.
        unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) }
    }
}

impl Drop for ShmemWriteBuffer<'_> {
    fn drop(&mut self) {
        if self.submitted > 0 {
            self.channel
                .tail
                .fetch_add(self.submitted, Ordering::Release);
            self.channel.notify_readable();
        }
    }
}
