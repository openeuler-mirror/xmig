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

use thiserror::Error;

/// Errors that can occur during bitwise read or write operations.
#[derive(Debug, Error)]
pub enum BytewiseError {
    #[error("Insufficient buffer capacity (required: {required}, capacity: {capacity})")]
    InsufficientBuffer { required: usize, capacity: usize },

    #[error("Alignment must be power of two (align: {align})")]
    InvalidAlignment { align: usize },

    #[error("Memory address cannot satisfy {align}-byte alignment")]
    IntrinsicMisalignment { align: usize },

    #[error("Illegal overlapping copy operation")]
    IllegalOverlappingCopy,
}

/// Provides a way to read raw, aligned data from a byte buffer.
///
/// This reader maintains an internal offset to keep track of the current
/// reading position. It ensures that data is read from memory addresses
/// that respect its type alignment.
///
/// It can operate on immutable (`&[u8]`) and mutable (`&mut [u8]`) buffers.
pub trait BytewiseReader<'a> {
    /// Returns the number of bytes successfully read from the buffer.
    ///
    /// This value increments after each successful read operation and can be used
    /// to track progress or validate read boundaries.
    ///
    /// # Returns
    /// - `usize` The total number of bytes consumed from the buffer so far.
    fn read_bytes(&self) -> usize;

    /// Reads a raw memory slice from the buffer with specified alignment.
    ///
    /// This is the most fundamental reading operation. It calculates the necessary padding
    /// to meet the alignment requirement, advances the internal offset, and returns a
    /// pointer to the aligned data start.
    ///
    /// # Parameters
    /// - `size`: The number of bytes to read.
    /// - `align`: The required alignment for the data's starting address within the buffer.
    ///
    /// # Returns
    /// - `Ok(NonNull<u8>)`: On success, an immutable raw pointer to the start of the aligned data.
    /// - `Err(BytewiseError)`: On failure, contains an error detailing the cause.
    ///
    /// # Safety
    /// This function is `unsafe` because it returns a raw pointer (`NonNull<u8>`) whose
    /// lifetime is not tracked by the compiler.
    ///
    /// To avoid undefined behavior, the caller **must** uphold the following contract:
    /// - The `size` and `align` parameters **must** accurately describe the data being read.
    /// - The returned pointer **must** be cast to the correct type before being dereferenced.
    /// - The returned pointer **must not** be used after the reader's lifetime ends.
    unsafe fn read_raw(
        &mut self,
        size: usize,
        align: usize,
    ) -> Result<ptr::NonNull<u8>, BytewiseError>;

    /// Reads a typed value from the buffer and returns an immutable reference to it.
    ///
    /// The returned reference's lifetime is tied to the lifetime of the reader's buffer.
    ///
    /// # Returns
    /// - `Ok(&'a T)`: On success, an immutable reference `&T` to the start of the aligned data.
    /// - `Err(BytewiseError)`: On failure, contains an error detailing the cause.
    ///
    /// # Safety
    /// This function is `unsafe` because it interprets raw bytes as a typed reference `&T`.
    ///
    /// To avoid undefined behavior, the caller **must** uphold the following contract:
    /// - The bytes at the current read position **must** represent for type `T`.
    unsafe fn read_ref<T: Copy>(&mut self) -> Result<&'a T, BytewiseError>;

    /// Reads a typed value from the buffer and returns a mutable reference to it.
    ///
    /// The returned reference's lifetime is tied to the lifetime of the reader's buffer.
    /// This allows for in-place modification of data within the buffer.
    ///
    /// # Returns
    /// - `Ok(&mut T)`: On success, a mutable reference `&T` to the start of the aligned data.
    /// - `Err(BytewiseError)`: On failure, contains an error detailing the cause.
    ///
    /// # Safety
    /// This function is `unsafe` because it creates an exclusive mutable reference
    /// `&mut T` from raw bytes.
    ///
    // To avoid undefined behavior, the caller **must** uphold the following contract:
    /// - The bytes at the current read position **must** represent for type `T`.
    /// - No other pointers or references to this memory location may exist for the
    ///   lifetime of the returned `&mut T`, upholding Rust's aliasing rules.
    unsafe fn read_mut<T: Copy>(&mut self) -> Result<&'a mut T, BytewiseError>;
}

/// Provides a way to write raw, aligned data into a byte buffer.
///
/// This writer maintains an internal offset to keep track of the current
/// writing position. It ensures that data is written at memory that
/// respect the requested alignment.
pub trait BytewiseWriter {
    /// Returns the current written bytes.
    ///
    /// # Returns
    /// * `usize`: Written bytes
    fn written_bytes(&self) -> usize;

    /// Writes a raw pointer into the buffer with specified alignment.
    ///
    /// This is the most fundamental writing operation.
    /// It calculates the necessary padding to meet the alignment requirement, then copies the data.
    ///
    /// # Parameters
    /// - `ptr`: A raw pointer to the data to be written.
    /// - `size`: The number of bytes to copy from the source.
    /// - `align`: The required alignment for the destination address within the buffer.
    ///
    /// # Returns
    /// - `Ok(())`: On success, returns nothing.
    /// - `Err(BytewiseError)`: On failure, contains an error detailing the cause.
    ///
    /// # Safety
    /// This function is `unsafe` because it operates on a raw pointer `ptr` and relies
    /// on the caller to guarantee the validity of the memory it describes.
    ///
    /// To avoid undefined behavior, the caller **must** uphold the following contract:
    /// - The data pointed by `ptr` **must** be valid for byte-level copy operations.
    /// - The data pointed by `ptr` **must** be valid for reads of `size` bytes.
    /// - The data pointed by `ptr` **must** be properly aligned to `align` bytes.
    /// - The data pointed by `ptr` **must** remain accessible for the duration of the operation.
    unsafe fn write_raw(
        &mut self,
        ptr: ptr::NonNull<u8>,
        size: usize,
        align: usize,
    ) -> Result<(), BytewiseError>;

    /// Writes a typed value from a reference into the buffer.
    ///
    /// This function uses a safe reference `&T` to guarantee the validity of the
    /// pointer passed to the underlying unsafe functions.
    ///
    /// # Parameters
    /// - `value`: A reference to the value to be written.
    ///
    /// # Returns
    /// - `Ok(())`: On success, returns nothing.
    /// - `Err(BytewiseError)`: On failure, contains an error detailing the cause (e.g., insufficient space).
    fn write_ref<T: Copy>(&mut self, value: &T) -> Result<(), BytewiseError>;
}

/// A trait for types that can be serialized into a byte-oriented writer.
///
/// Implementors of this trait define how their structure and data are written
/// sequentially into a `ByteWriter`, which abstracts the underlying buffer.
pub trait BytewiseWrite {
    /// Serializes the object into the provided byte writer.
    ///
    /// This method is the core of the `BytewiseWrite` trait. Implementors should
    /// use the `writer` to serialize their fields in a deterministic order. The
    /// `ByteWriter` handles the low-level details of buffer management, such as
    /// tracking the current position and checking for capacity limits.
    ///
    /// # Parameters
    /// - `writer`: A mutable reference to a `ByteWriter` implementation, which serves
    ///   as the destination for the serialized data.
    ///
    /// # Returns
    /// - `Ok(())`: On successful serialization.
    /// - `Err(BytewiseError)`: If an error occurs during writing, such as the
    ///   buffer running out of space (`BufferTooSmall`).
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError>;
}

/// A trait for types that support zero-copy deserialization from a byte reader.
///
/// This trait is designed for performance-critical scenarios where allocation and
/// data copying can be avoided. The deserialized type is returned as a reference
/// that borrows directly from the input buffer, ensuring no data is moved.
pub trait BytewiseRead {
    /// Deserializes a reference to `Self` from the given reader without copying.
    ///
    /// This method enables zero-copy deserialization by returning a reference (`&'a Self`)
    /// that is valid for the lifetime `'a` of the reader's buffer. It is ideal for
    /// types that can be safely represented as a view into the byte buffer, such as
    /// structs containing `&'a str` or `&'a [u8]`.
    ///
    /// # Parameters
    /// - `reader`: A mutable reference to a `ByteRefReader` which provides a view
    ///   into the source byte buffer.
    ///
    /// # Returns
    /// - `Ok(&'a Self)`: On success, returns a borrowed reference to the deserialized object.
    /// - `Err(BytewiseError)`: If an error occurs, such as insufficient data in the
    ///   buffer (`BufferTooSmall`) or malformed data.
    fn read_ref<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<&'a Self, BytewiseError>;
}

/// A trait for types that are deserialized into an owned value.
///
/// This trait is used for the standard deserialization pattern where data is copied
/// from the source buffer into a new, independent object. The resulting object has
/// no lifetime dependency on the input buffer.
pub trait BytewiseReadOwned: Sized {
    /// Deserializes an owned instance of `Self` by copying data from the reader.
    ///
    /// This method creates a new, owned instance of the type by reading from the
    /// provided `reader`. It is the standard approach for types that own their data
    /// (e.g., `String`, `Vec<T>`) and do not borrow from the input buffer.
    ///
    /// # Parameters
    /// - `reader`: A mutable reference to a `ByteRefReader` which provides a view
    ///   into the source byte buffer.
    ///
    /// # Returns
    /// - `Ok(Self)`: On success, returns a new, owned instance of the object.
    /// - `Err(BytewiseError)`: If an error occurs during reading, such as the buffer
    ///   being too small or the data being invalid.
    fn read_from<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError>;

    /// Deserializes an owned instance of `Self` from a mutable reader.
    ///
    /// This function is functionally equivalent to `read_from` but accepts a mutable
    /// reader. It provides symmetry with the `BytewiseRead` trait and can be useful
    /// in generic contexts where the reader might be mutable for other reasons.
    ///
    /// # Parameters
    /// - `reader`: A mutable reference to a `ByteMutReader`, providing access to the
    ///   source byte buffer.
    ///
    /// # Returns
    /// - `Ok(Self)`: On success, returns a new, owned instance of the object.
    /// - `Err(BytewiseError)`: If an error occurs during reading.
    fn read_from_mut<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError>;
}

mod buffer;
pub use buffer::*;

#[cfg(test)]
mod tests;
