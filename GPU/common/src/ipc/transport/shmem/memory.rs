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

use std::io;

use nix::errno::Errno;

use crate::sys::{mmap::MirroredMmap, page, shmem::Shmem};

#[derive(Debug)]
pub struct ShmemRegion {
    shmem: Shmem,
    mapping: MirroredMmap,
}

impl ShmemRegion {
    pub fn create<S: AsRef<str>>(name: S, len: usize, reserve: usize) -> io::Result<Self> {
        let data_len = page::page_align(len);
        let resv_len = page::page_align(reserve);
        let file_len = data_len.checked_add(resv_len).ok_or(Errno::EINVAL)?;

        let shmem = Shmem::create(name, file_len)?;
        let mapping = MirroredMmap::mmap_from(&shmem, file_len, resv_len)?;

        Ok(Self { shmem, mapping })
    }

    pub fn open<S: AsRef<str>>(name: S, reserved: usize) -> io::Result<Self> {
        let resv_len = page::page_align(reserved);

        let shmem = Shmem::open(name)?;
        let mapping = MirroredMmap::mmap_from(&shmem, shmem.size(), resv_len)?;

        Ok(Self { shmem, mapping })
    }

    #[inline]
    pub fn name(&self) -> &str {
        self.shmem.name()
    }

    #[inline]
    pub fn is_owner(&self) -> bool {
        self.shmem.is_owner()
    }

    /// Returns a pointer to the start of the reserved region (A).
    #[inline]
    pub fn reserved_ptr(&self) -> *mut u8 {
        self.mapping.reserved_ptr()
    }

    /// Returns a pointer to the start of the main data region (B).
    #[inline]
    pub fn data_ptr(&self) -> *mut u8 {
        self.mapping.data_ptr()
    }

    /// Returns the length of the main data region (B).
    #[inline]
    pub fn data_len(&self) -> usize {
        self.mapping.data_len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mirrored_lifecycle() -> std::io::Result<()> {
        const SHM_NAME: &str = "/test_mirrored_lifecycle";

        const SHMEM_LEN: usize = 8192;
        const RESERVED_LEN: usize = 4096;
        const TEST_BYTE: u8 = 0xF0;

        let shmem1 = ShmemRegion::create(SHM_NAME, SHMEM_LEN, RESERVED_LEN).unwrap();
        println!("shmem1: {:#?}", shmem1.mapping);

        assert!(shmem1.is_owner());

        let expected_reserved_len = page::page_align(RESERVED_LEN);
        let expected_file_len = page::page_align(SHMEM_LEN + expected_reserved_len);
        let expected_data_len = expected_file_len - expected_reserved_len;

        assert_eq!(shmem1.mapping.reserved_len(), expected_reserved_len);
        assert_eq!(shmem1.data_len(), expected_data_len);
        assert_eq!(
            shmem1.mapping.total_len(),
            expected_reserved_len + expected_data_len * 2
        );

        unsafe {
            std::ptr::write_bytes(shmem1.data_ptr(), TEST_BYTE, shmem1.data_len());
        }

        let shmem2 = ShmemRegion::open(SHM_NAME, RESERVED_LEN)?;
        println!("shmem2: {:#?}", shmem2.mapping);

        assert!(!shmem2.is_owner()); // Opener is not the owner.
        assert_eq!(shmem1.name(), shmem2.name());

        assert_ne!(shmem1.reserved_ptr(), shmem2.reserved_ptr());
        assert_ne!(shmem1.data_ptr(), shmem2.data_ptr());
        assert_eq!(shmem1.data_len(), shmem2.data_len());
        assert_eq!(shmem1.mapping.reserved_len(), shmem2.mapping.reserved_len());
        assert_eq!(shmem1.mapping.total_len(), shmem2.mapping.total_len());

        let shmem1_data_slice =
            unsafe { std::slice::from_raw_parts(shmem1.data_ptr(), shmem1.data_len()) };
        let shmem2_data_slice =
            unsafe { std::slice::from_raw_parts(shmem2.data_ptr(), shmem2.data_len()) };

        assert_eq!(shmem1_data_slice, shmem2_data_slice);

        assert!(!shmem2_data_slice.is_empty());
        assert_eq!(shmem2_data_slice[0], TEST_BYTE);
        assert!(shmem2_data_slice.iter().all(|&b| b == TEST_BYTE));

        Ok(())
    }

    #[test]
    fn test_basic_mirroring() -> io::Result<()> {
        const SHM_NAME: &str = "/test_basic_mirroring";
        const SHM_LEN: usize = 8192;
        const RESERVED_LEN: usize = 4096;

        let shmem = ShmemRegion::create(SHM_NAME, SHM_LEN, RESERVED_LEN)?;

        let data_len = shmem.data_len();
        let data_ptr = shmem.data_ptr();
        let mirrored_ptr = shmem.mapping.mirrored_ptr();

        // Scenario 1: Write to the data region (B) and read from the mirrored region (B')
        unsafe {
            // Write data at different positions in region B
            *data_ptr.add(0) = 0x11;
            *data_ptr.add(100) = 0x22;
            *data_ptr.add(data_len - 1) = 0x33;

            // Verify that the data at the same offsets in region B' matches
            assert_eq!(*mirrored_ptr.add(0), 0x11);
            assert_eq!(*mirrored_ptr.add(100), 0x22);
            assert_eq!(*mirrored_ptr.add(data_len - 1), 0x33);
        }

        // Scenario 2: Write to the mirrored region (B') and read from the data region (B)
        unsafe {
            // Write data at a specific position in region B'
            *mirrored_ptr.add(55) = 0x44;

            // Verify that the data at the same offset in region B matches
            assert_eq!(*data_ptr.add(55), 0x44);
        }

        Ok(())
    }

    #[test]
    fn test_wrap_around_write() -> io::Result<()> {
        const SHM_NAME: &str = "/test_wrap_around_write";
        const SHM_LEN: usize = 8192;
        const RESERVED_LEN: usize = 4096;

        let shmem = ShmemRegion::create(SHM_NAME, SHM_LEN, RESERVED_LEN)?;

        let data_len = shmem.data_len();
        let data_ptr = shmem.data_ptr();

        // Define a small chunk of data for the wrap-around write.
        let wrap_around_data = [0xDE, 0xAD, 0xBE, 0xEF];

        // Choose an offset near the end of region B to start writing from.
        let start_offset = data_len - 2;

        unsafe {
            // Since B and B' are contiguous in virtual memory, this single copy operation
            // will seamlessly write to the last two bytes of region B and continue to the
            // first two bytes of region B'.
            let write_slice =
                std::slice::from_raw_parts_mut(data_ptr.add(start_offset), wrap_around_data.len());
            write_slice.copy_from_slice(&wrap_around_data);

            // Now, verify the most critical part:
            // The data written to the start of the mirrored region (B') should be readable
            // from the start of the main data region (B).

            // Expected memory state:
            // B[data_len - 2] = 0xDE
            // B[data_len - 1] = 0xAD
            // B'[0] (physically the same as B[0]) = 0xBE
            // B'[1] (physically the same as B[1]) = 0xEF

            // Verify the data at the end of region B
            assert_eq!(*data_ptr.add(data_len - 2), 0xDE);
            assert_eq!(*data_ptr.add(data_len - 1), 0xAD);

            // The most important verification: check that the start of region B has been updated.
            assert_eq!(*data_ptr.add(0), 0xBE);
            assert_eq!(*data_ptr.add(1), 0xEF);
        }

        Ok(())
    }
}
