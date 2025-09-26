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

use core::ptr;

use super::{BytewiseError, BytewiseReader, BytewiseWriter};

pub struct BytewiseBuffer<B> {
    buf: B,
    read: usize,
    write: usize,
}

impl<B> BytewiseBuffer<B> {
    /// Creates a new `BytewiseBuffer` that reads from / writes to the provided buffer.
    ///
    /// # Parameters
    /// - `buf`: The byte slice that this reader will read from.
    ///
    /// # Returns
    /// - `Self`: new `BytewiseBuffer` instance.
    pub fn new(buf: B) -> Self {
        Self {
            buf,
            read: 0,
            write: 0,
        }
    }

    /// Creates a new `BytewiseBuffer` that reads from / writes to the provided buffer.
    ///
    /// # Parameters
    /// - `buf`: The byte slice that this reader will read from.
    /// - `read`: The initial read position in bytes from the start of the buffer.
    /// - `write`: The initial write position in bytes from the start of the buffer.
    ///
    /// # Returns
    /// - `Self`: new `ByteStream` instance.
    pub fn with_offset(buf: B, read: usize, write: usize) -> Self {
        Self { buf, read, write }
    }

    /// Consumes the `BytewiseBuffer` and returns the underlying buffer.
    ///
    /// # Returns
    /// - The original buffer of type `B` that was used to create this `BytewiseBuffer`.
    ///   The returned buffer will contain any modifications made through write operations.
    pub fn into_inner(self) -> B {
        self.buf
    }
}

impl<'a, B: AsRef<[u8]>> BytewiseReader<'a> for BytewiseBuffer<B> {
    fn read_bytes(&self) -> usize {
        self.read
    }

    unsafe fn read_ptr(&mut self, size: usize, align: usize) -> Result<*const u8, BytewiseError> {
        // For zero-sized types, return a dangling pointer and consume 0 bytes.
        if size == 0 {
            return Ok(ptr::NonNull::dangling().as_ptr());
        }

        // Check alignment
        if !align.is_power_of_two() {
            return Err(BytewiseError::InvalidAlignment { align });
        }

        // Get remaining readable buffer
        let rest_buf = self
            .buf
            .as_ref()
            .get(self.read..)
            .ok_or(BytewiseError::BufferTooSmall {
                required: size,
                capacity: 0,
            })?;

        // Calculate padding for alignment
        let padding_size = rest_buf.as_ptr().align_offset(align);
        if padding_size == usize::MAX {
            return Err(BytewiseError::AlignmentError { align });
        }

        // Calculate the total size this op requires
        let total_size = padding_size.saturating_add(size);

        // Check if the remaining buffer is large enough
        let rest_size = rest_buf.len();
        if total_size > rest_size {
            return Err(BytewiseError::BufferTooSmall {
                required: total_size,
                capacity: rest_size,
            });
        }

        // Calculate aligned readable data position
        let data_ptr = rest_buf[padding_size..].as_ptr();

        // Advance the internal offset
        self.read += total_size;

        Ok(data_ptr)
    }

    unsafe fn read_ref<T>(&mut self) -> Result<&'a T, BytewiseError> {
        // SAFETY: The call to `unsafe fn self.read_raw_ptr` is safe because:
        // - The `size` and `align` is accurately describes the data being read (compiler guarantee)
        // - The returned pointer is casted to correct type (safety contract)
        // - The returned pointer's lifetime follows the reader's lifetime (compiler guarantee)
        unsafe {
            self.read_ptr(size_of::<T>(), align_of::<T>())
                .map(|ptr| &*ptr.cast())
        }
    }

    unsafe fn read_mut_ptr(&mut self, size: usize, align: usize) -> Result<*mut u8, BytewiseError> {
        // SAFETY: The call to `unsafe fn self.read_raw_ptr` is safe because:
        // - This function's safety contract requires its caller to uphold the safety
        //   contract of `read_raw_ptr` (safety contract).
        unsafe { self.read_ptr(size, align).map(|ptr| ptr.cast_mut()) }
    }

    unsafe fn read_mut<T>(&mut self) -> Result<&'a mut T, BytewiseError> {
        // SAFETY: The call to `unsafe fn self.read_raw_mut_ptr` is safe because:
        // - The `size` and `align` is accurately describes the data being read (compiler guarantee)
        // - The returned pointer is casted to correct type (safety contract)
        // - The returned pointer's lifetime follows the reader's lifetime (compiler guarantee)
        // - The returned pointer would not violate Rust's aliasing rules (safety contract)
        unsafe {
            self.read_mut_ptr(size_of::<T>(), align_of::<T>())
                .map(|ptr| &mut *ptr.cast())
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> BytewiseWriter for BytewiseBuffer<B> {
    fn written_bytes(&self) -> usize {
        self.write
    }

    unsafe fn write_raw_ptr(
        &mut self,
        ptr: *const u8,
        size: usize,
        align: usize,
    ) -> Result<(), BytewiseError> {
        // For zero-sized writes, perform a no-op and return success
        if size == 0 {
            return Ok(());
        }

        // Check pointer
        if ptr.is_null() {
            return Err(BytewiseError::NullPointer);
        }

        // Check alignment
        if !align.is_power_of_two() {
            return Err(BytewiseError::InvalidAlignment { align });
        }

        // Get remaining writable buffer
        let buf_len = self.buf.as_ref().len();
        let rest_buf =
            self.buf
                .as_mut()
                .get_mut(self.write..)
                .ok_or(BytewiseError::BufferTooSmall {
                    required: self.write + 1,
                    capacity: buf_len,
                })?;

        // Calculate padding for alignment
        let padding_size = rest_buf.as_ptr().align_offset(align);
        if padding_size == usize::MAX {
            return Err(BytewiseError::AlignmentError { align });
        }

        // Calculate the total size this op requires
        let total_size = padding_size.saturating_add(size);

        // Check if the remaining buffer is large enough
        let rest_len = rest_buf.len();
        if total_size > rest_len {
            return Err(BytewiseError::BufferTooSmall {
                required: self.write + total_size,
                capacity: buf_len,
            });
        }

        // Calculate aligned writable data position
        let dst_ptr = rest_buf[padding_size..].as_mut_ptr();
        let src_ptr = ptr;

        // Enforce the `ptr::copy_nonoverlapping` requirement
        let src_start = src_ptr as usize;
        let src_end = src_start.saturating_add(size);
        let dst_start = dst_ptr as usize;
        let dst_end = dst_start.saturating_add(size);
        if src_start < dst_end && src_end > dst_start {
            return Err(BytewiseError::MemoryOverlap);
        }

        // Write data to aligned position
        // SAFETY: This `ptr::copy_nonoverlapping` call is safe because:
        // - Source pointer validity is guaranteed by caller (safety contract)
        // - Source pointer alignment is guaranteed by caller (safety contract)
        // - Destination pointer validity is verified via bounds check (runtime check)
        // - Destination pointer alignment is enforced via `align_offset` call (runtime check)
        // - Non-overlapping between source and destination is verified via address comparison (runtime check)
        unsafe {
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, size);
        }

        // Advance the internal offset
        self.write += total_size;

        Ok(())
    }

    unsafe fn write_ptr<T>(&mut self, ptr: *const T) -> Result<(), BytewiseError> {
        // SAFETY: The call to `unsafe fn self.write_raw_ptr` is safe because:
        // - The type and lifetime of the data pointed to by `ptr` are guaranteed by the caller (safety contract)
        // - The `size` and `align` arguments are correctly derived from `T` (compiler guarantee)
        unsafe { self.write_raw_ptr(ptr.cast::<u8>(), size_of::<T>(), align_of::<T>()) }
    }

    unsafe fn write_ref<T>(&mut self, value: &T) -> Result<(), BytewiseError> {
        // SAFETY: The call to `unsafe fn self.write_ptr` is safe because:
        // - The type and lifetime of the pointer are guaranteed by the `&T` reference (compiler guarantee)
        // - The byte-level validity of `T` is guaranteed by the caller (safety contract)
        unsafe { self.write_ptr(ptr::from_ref(value)) }
    }
}
