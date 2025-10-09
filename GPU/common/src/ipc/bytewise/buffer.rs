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
    /// - `Self`: new `BytewiseBuffer` instance.
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

    unsafe fn read_raw(
        &mut self,
        size: usize,
        align: usize,
    ) -> Result<ptr::NonNull<u8>, BytewiseError> {
        // For zero-sized types, return a dangling pointer and consume 0 bytes.
        if size == 0 {
            return Ok(ptr::NonNull::dangling());
        }

        // Check alignment
        if !align.is_power_of_two() {
            return Err(BytewiseError::InvalidAlignment { align });
        }

        // Get remaining readable buffer
        let rest_buf =
            self.buf
                .as_ref()
                .get(self.read..)
                .ok_or(BytewiseError::InsufficientBuffer {
                    required: size,
                    capacity: 0,
                })?;

        // Calculate padding for alignment
        let padding_size = rest_buf.as_ptr().align_offset(align);
        if padding_size == usize::MAX {
            return Err(BytewiseError::IntrinsicMisalignment { align });
        }

        // Calculate the total size this op requires
        let total_size = padding_size.saturating_add(size);

        // Check if the remaining buffer is large enough
        let rest_size = rest_buf.len();
        if total_size > rest_size {
            return Err(BytewiseError::InsufficientBuffer {
                required: total_size,
                capacity: rest_size,
            });
        }

        // Calculate aligned readable data position
        let buf_ptr = rest_buf[padding_size..].as_ptr().cast_mut();
        let data_ptr = ptr::NonNull::new(buf_ptr).expect("Unexpected null pointer from slice");

        // Advance the internal offset
        self.read += total_size;

        Ok(data_ptr)
    }

    unsafe fn read_ref<T: Copy>(&mut self) -> Result<&'a T, BytewiseError> {
        // SAFETY: This `read_raw` call is safe because:
        // - The `size` and `align` is accurately describes the data being read (compiler guarantee)
        // - The returned reference is casted to correct type (safety contract)
        // - The returned reference lifetime follows the data lifetime (compiler guarantee)
        unsafe {
            self.read_raw(size_of::<T>(), align_of::<T>())
                .map(|ptr| ptr.cast().as_ref())
        }
    }

    unsafe fn read_mut<T: Copy>(&mut self) -> Result<&'a mut T, BytewiseError> {
        // SAFETY: This `read_raw` call is safe because:
        // - The `size` and `align` is accurately describes the data being read (compiler guarantee)
        // - The returned reference is casted to correct type (safety contract)
        // - The returned reference lifetime follows the data lifetime (compiler guarantee)
        // - The returned reference would not violate Rust's aliasing rules (safety contract)
        unsafe {
            self.read_raw(size_of::<T>(), align_of::<T>())
                .map(|ptr| ptr.cast().as_mut())
        }
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> BytewiseWriter for BytewiseBuffer<B> {
    fn written_bytes(&self) -> usize {
        self.write
    }

    unsafe fn write_raw(
        &mut self,
        ptr: ptr::NonNull<u8>,
        size: usize,
        align: usize,
    ) -> Result<(), BytewiseError> {
        // For zero-sized writes, perform a no-op and return success
        if size == 0 {
            return Ok(());
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
                .ok_or(BytewiseError::InsufficientBuffer {
                    required: self.write + 1,
                    capacity: buf_len,
                })?;

        // Calculate padding for alignment
        let padding_size = rest_buf.as_ptr().align_offset(align);
        if padding_size == usize::MAX {
            return Err(BytewiseError::IntrinsicMisalignment { align });
        }

        // Calculate the total size this op requires
        let total_size = padding_size.saturating_add(size);

        // Check if the remaining buffer is large enough
        let rest_len = rest_buf.len();
        if total_size > rest_len {
            return Err(BytewiseError::InsufficientBuffer {
                required: self.write + total_size,
                capacity: buf_len,
            });
        }

        // Calculate aligned writable data position
        let dst_ptr = rest_buf[padding_size..].as_mut_ptr();
        let src_ptr = ptr.as_ptr().cast_const();

        // Enforce the `ptr::copy_nonoverlapping` requirement
        let src_start = src_ptr as usize;
        let src_end = src_start.saturating_add(size);
        let dst_start = dst_ptr as usize;
        let dst_end = dst_start.saturating_add(size);
        if src_start < dst_end && src_end > dst_start {
            return Err(BytewiseError::IllegalOverlappingCopy);
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

    fn write_ref<T: Copy>(&mut self, value: &T) -> Result<(), BytewiseError> {
        // SAFETY: This `write_raw` call is safe because:
        // - The source pointer is valid for byte-level copy (compiler guarantee)
        // - The source data is valid for reads of `size_of::<T>()` bytes (compiler guarantee)
        // - The source pointer is properly aligned to `align_of::<T>()` (compiler guarantee)
        // - The source data remains accessible during operation (compiler guarantee)
        unsafe {
            self.write_raw(
                ptr::NonNull::from_ref(value).cast(),
                size_of::<T>(),
                align_of::<T>(),
            )
        }
    }
}
