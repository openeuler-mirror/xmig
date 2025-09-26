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

use core::{ffi::c_void, num::NonZero, ptr::NonNull};
use std::{io, os::fd::AsFd};

use nix::{errno::Errno, sys::mman as mm};

use super::page;

#[derive(Debug)]
struct MmapGuard {
    ptr: NonNull<c_void>,
    len: usize,
}

impl Drop for MmapGuard {
    fn drop(&mut self) {
        let _ = unsafe { mm::munmap(self.ptr, self.len) };
    }
}

pub struct Mmap {
    mmap: MmapGuard,
}

impl Mmap {
    /// Creates a new memory mapping from a file descriptor.
    ///
    /// - `fd`: The file descriptor to map.
    /// - `len`: The total length of the file to be mapped.
    pub fn mmap_from<F: AsFd>(fd: &F, len: usize) -> io::Result<Self> {
        debug_assert!(
            len % page::page_size() == 0,
            "Mmap::mmap_from requires 'len' ({}) to be page-aligned",
            len
        );

        let ptr = unsafe {
            mm::mmap(
                None,
                NonZero::new(len).ok_or(Errno::EINVAL)?,
                mm::ProtFlags::PROT_NONE,
                mm::MapFlags::MAP_ANONYMOUS | mm::MapFlags::MAP_PRIVATE,
                fd.as_fd(),
                0,
            )
        }?;

        let mmap = Self {
            mmap: MmapGuard { ptr, len },
        };
        Ok(mmap)
    }

    /// Returns a pointer to the start of the entire mapping.
    #[inline]
    pub fn ptr(&self) -> *mut u8 {
        self.mmap.ptr.cast().as_ptr()
    }

    /// Returns the total size of the entire virtual mapped region.
    #[inline]
    pub fn len(&self) -> usize {
        self.mmap.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.mmap.len == 0
    }
}

/// A struct that provides a mirrored (virtually contiguous) memory mapping.
///
/// The memory layout is as follows:
/// `[ A: Reserved | B: Data | B': Mirror of Data ]`
///
/// This allows a ring buffer implemented on top of this to perform wrap-around
/// operations with a single `memcpy`.
#[derive(Debug)]
pub struct MirroredMmap {
    mmap: MmapGuard,
    resv_len: usize,
    data_len: usize,
}

impl MirroredMmap {
    /// Creates a new mirrored memory mapping from a file descriptor.
    ///
    /// - `fd`: The file descriptor to map.
    /// - `len`: The total length of the file to be mapped (`A + B`).
    /// - `resv_len`: The size of the header / reserved area (`A`).
    pub fn mmap_from<F: AsFd>(fd: &F, len: usize, resv_len: usize) -> io::Result<Self> {
        debug_assert!(
            len % page::page_size() == 0,
            "MirroredMmap::mmap_from requires 'len' ({}) to be page-aligned",
            len
        );
        debug_assert!(
            resv_len % page::page_size() == 0,
            "MirroredMmap::mmap_from requires 'resv_len' ({}) to be page-aligned",
            resv_len
        );

        if resv_len == 0 {
            return Err(Errno::EINVAL.into());
        }
        if resv_len >= len {
            return Err(Errno::EINVAL.into());
        }

        let data_len = len.saturating_sub(resv_len);
        let total_len = resv_len.saturating_add(data_len.saturating_mul(2));

        let mmap = unsafe {
            let mmap_start = mm::mmap_anonymous(
                None,
                NonZero::new(total_len).ok_or(Errno::EINVAL)?,
                mm::ProtFlags::PROT_NONE,
                mm::MapFlags::MAP_ANONYMOUS | mm::MapFlags::MAP_PRIVATE,
            )?;
            let mmap_guard = MmapGuard {
                ptr: mmap_start,
                len: total_len,
            };

            let mmap_ptr = mm::mmap(
                Some(mmap_start.addr()),
                NonZero::new(len).ok_or(Errno::EINVAL)?,
                mm::ProtFlags::PROT_READ | mm::ProtFlags::PROT_WRITE,
                mm::MapFlags::MAP_SHARED | mm::MapFlags::MAP_FIXED,
                fd,
                0,
            )?;
            if mmap_ptr != mmap_start {
                return Err(Errno::EADDRNOTAVAIL.into());
            }

            let mirror_start = mmap_ptr.byte_add(len);
            let mirrored_ptr = mm::mmap(
                Some(mirror_start.addr()),
                NonZero::new(data_len).ok_or(Errno::EINVAL)?,
                mm::ProtFlags::PROT_READ | mm::ProtFlags::PROT_WRITE,
                mm::MapFlags::MAP_SHARED | mm::MapFlags::MAP_FIXED,
                fd,
                resv_len.try_into().map_err(|_| Errno::EINVAL)?,
            )?;
            if mirrored_ptr != mirror_start {
                return Err(Errno::EADDRNOTAVAIL.into());
            }

            mmap_guard
        };

        Ok(Self {
            mmap,
            resv_len,
            data_len,
        })
    }

    /// Returns a pointer to the start of the entire mapping (Region A).
    #[inline]
    pub fn base_ptr(&self) -> *mut u8 {
        self.mmap.ptr.cast().as_ptr()
    }

    /// Returns a pointer to the start of the reserved region (A).
    #[inline]
    pub fn reserved_ptr(&self) -> *mut u8 {
        self.base_ptr()
    }

    /// Returns the length of the reserved region (A).
    #[inline]
    pub fn reserved_len(&self) -> usize {
        self.resv_len
    }

    /// Returns a pointer to the start of the main data region (B).
    #[inline]
    pub fn data_ptr(&self) -> *mut u8 {
        unsafe { self.base_ptr().byte_add(self.resv_len) }
    }

    /// Returns the length of the data region (B).
    #[inline]
    pub fn data_len(&self) -> usize {
        self.data_len
    }

    /// Returns a pointer to the start of the mirrored data region (B').
    #[inline]
    pub fn mirrored_ptr(&self) -> *mut u8 {
        unsafe { self.data_ptr().byte_add(self.data_len) }
    }

    /// Returns the length of the mirrored region (B').
    #[inline]
    pub fn mirrored_len(&self) -> usize {
        self.data_len
    }

    /// Returns the total size of the entire virtual mapped region (A + B + B').
    #[inline]
    pub fn total_len(&self) -> usize {
        self.mmap.len
    }
}

#[cfg(test)]
mod tests {
    use std::io;

    use crate::sys::shmem::Shmem;

    use super::*;

    #[test]
    fn test_mirrored_mmap_basic() -> io::Result<()> {
        const MAP_SIZE: usize = 8192;
        const RESERVE_SIZE: usize = 4096;
        const DATA_SIZE: usize = MAP_SIZE - RESERVE_SIZE;
        const TOTAL_SIZE: usize = RESERVE_SIZE + DATA_SIZE * 2;

        let shmem = Shmem::create("/test_mirrored_mmap", MAP_SIZE)?;
        let mmap = MirroredMmap::mmap_from(&shmem, MAP_SIZE, RESERVE_SIZE)?;

        assert_eq!(mmap.reserved_len(), RESERVE_SIZE);
        assert_eq!(mmap.data_len(), DATA_SIZE);
        assert_eq!(mmap.mirrored_len(), DATA_SIZE);
        assert_eq!(mmap.total_len(), TOTAL_SIZE);

        unsafe {
            let data = mmap.data_ptr();
            let mirror = mmap.mirrored_ptr();

            *data = 0xAA;
            assert_eq!(*mirror, 0xAA);

            *data.byte_add(100) = 0xBB;
            assert_eq!(*mirror.byte_add(100), 0xBB);
        }

        Ok(())
    }
}
