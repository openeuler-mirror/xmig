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
    any::{TypeId, type_name},
    fmt::Debug,
    marker::PhantomData,
    ptr, slice,
};

use bitflags::bitflags;

use crate::ipc::bytewise::{
    BytewiseError, BytewiseReadOwned, BytewiseReader, BytewiseWrite, BytewiseWriter,
};

use super::MessageError;

const INLINED_DATA_SIZE: usize = 16;
const INLINED_DATA_ALIGN: usize = 16;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ArgumentFlag: u32 {
        const ARG_IN = 0b0001;
        const ARG_OUT = 0b0010;
        const ARG_VIRT = 0b0100;
    }
}

impl Default for ArgumentFlag {
    fn default() -> Self {
        Self::ARG_IN
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArgumentKind {
    Scalar,
    Slice,
}

#[derive(Clone, Copy)]
struct ArgumentMetadata {
    kind: ArgumentKind,
    type_id: TypeId,
    type_size: usize,
    type_align: usize,
    len: usize,
}

impl Debug for ArgumentMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ArgumentMetadata")
            .field("type_id", &self.type_id)
            .field("type_size", &self.type_size)
            .field("type_align", &self.type_align)
            .field("len", &self.len)
            .finish()
    }
}

#[derive(Clone, Copy)]
#[repr(align(16))]
struct InlineBytes([u8; INLINED_DATA_SIZE]);

impl Debug for InlineBytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const PREVIEW_LEN: usize = 4;

        write!(f, "[")?;
        for (i, &byte) in self.0.iter().take(PREVIEW_LEN).enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "0x{:02x}", byte)?;
        }
        if self.0.len() > PREVIEW_LEN {
            write!(f, ", ...")?;
        }
        write!(f, "]")
    }
}

#[derive(Clone, Copy)]
enum ArgumentValue<'a> {
    Val(InlineBytes),
    Ref(ptr::NonNull<u8>, PhantomData<&'a ()>),
    Mut(ptr::NonNull<u8>, PhantomData<&'a mut ()>),
}

impl Debug for ArgumentValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Val(inline_bytes) => f.debug_struct("Val").field("data", inline_bytes).finish(),
            Self::Ref(ptr, _) => f.debug_struct("Ref").field("ptr", ptr).finish(),
            Self::Mut(ptr, _) => f.debug_struct("Mut").field("ptr", ptr).finish(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Argument<'a> {
    meta: ArgumentMetadata,
    value: ArgumentValue<'a>,
    flag: ArgumentFlag,
}

impl Argument<'_> {
    #[inline]
    pub const fn type_id(&self) -> TypeId {
        self.meta.type_id
    }

    #[inline]
    pub const fn type_size(&self) -> usize {
        self.meta.type_size
    }

    #[inline]
    pub const fn type_align(&self) -> usize {
        self.meta.type_align
    }

    #[inline]
    pub const fn len(&self) -> usize {
        self.meta.len
    }

    #[inline]
    pub const fn total_size(&self) -> usize {
        self.meta.type_size.saturating_mul(self.meta.len)
    }

    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.meta.type_size == 0 || self.meta.len == 0
    }

    #[inline]
    pub const fn flag(&self) -> ArgumentFlag {
        self.flag
    }
}

impl Argument<'_> {
    #[inline]
    pub fn empty() -> Argument<'static> {
        Self::from_value((), ArgumentFlag::default())
    }

    #[inline]
    pub fn from_value<T: Copy + 'static>(value: T, flag: ArgumentFlag) -> Argument<'static> {
        let meta = ArgumentMetadata {
            type_id: TypeId::of::<T>(),
            kind: ArgumentKind::Scalar,
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: 1,
        };
        if meta.type_size > INLINED_DATA_SIZE {
            panic!(
                "Type '{}' size {} exceeds {}-byte limit",
                type_name::<T>(),
                meta.type_size,
                INLINED_DATA_SIZE,
            );
        }
        if meta.type_align > INLINED_DATA_ALIGN {
            panic!(
                "Type '{}' alignment {} exceeds {}-byte limit",
                type_name::<T>(),
                meta.type_align,
                INLINED_DATA_SIZE,
            );
        }

        // For ZST, uses dangling pointer to save memory
        if meta.type_size == 0 {
            let value = ArgumentValue::Mut(ptr::NonNull::dangling(), PhantomData);
            return Argument { meta, value, flag };
        }

        let mut bytes = InlineBytes([0u8; INLINED_DATA_SIZE]);
        unsafe {
            ptr::copy_nonoverlapping(
                ptr::from_ref(&value).cast(),
                bytes.0.as_mut_ptr(),
                meta.type_size,
            );
        }
        let value = ArgumentValue::Val(bytes);

        Argument { meta, value, flag }
    }

    /// Creates an `Argument` from a raw constant pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer `ptr` adheres to the following conditions,
    /// which are necessary to create a valid shared reference `&'a T`:
    ///
    /// 1. `ptr` must be non-null and point to a properly initialized value of type `T`.
    /// 2. The memory pointed to by `ptr` must be valid for reads for the entire lifetime `'a`.
    /// 3. For the entire lifetime `'a`, the data pointed to by `ptr` must **not be mutated**
    ///    through any other pointer or reference. Multiple shared references are allowed, but
    ///    any form of mutable access is forbidden.
    ///
    /// Failure to uphold these guarantees will result in **undefined behavior**.
    #[inline]
    pub unsafe fn from_ptr<'a, T: 'static>(ptr: *const T, flag: ArgumentFlag) -> Argument<'a> {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Scalar,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: 1,
        };
        let value = ArgumentValue::Ref(
            {
                let ptr = ptr::NonNull::new(ptr.cast_mut().cast()).expect("Invalid null pointer");
                if meta.type_size != 0 && ptr == ptr::NonNull::dangling() {
                    panic!("Invalid dangling pointer");
                }
                ptr
            },
            PhantomData,
        );

        Argument { meta, value, flag }
    }

    /// Creates an `Argument` from a raw mutable pointer.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the pointer `ptr` adheres to the following conditions,
    /// which are necessary to create a valid unique mutable reference `&'a mut T`:
    ///
    /// 1. `ptr` must be non-null and point to a properly initialized value of type `T`.
    /// 2. The memory pointed to by `ptr` must be valid for both reads and writes for the
    ///    entire lifetime `'a`.
    /// 3. For the entire lifetime `'a`, **no other pointers or references (read or write)**
    ///    may access the data pointed to by `ptr`. This `Argument` assumes it has
    ///    exclusive access to the data.
    ///
    /// Failure to uphold these guarantees will result in **undefined behavior**.
    #[inline]
    pub unsafe fn from_mut_ptr<'a, T: 'static>(ptr: *mut T, flag: ArgumentFlag) -> Argument<'a> {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Scalar,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: 1,
        };
        let value = ArgumentValue::Mut(
            {
                let ptr = ptr::NonNull::new(ptr.cast()).expect("Invalid null pointer");
                if meta.type_size != 0 && ptr == ptr::NonNull::dangling() {
                    panic!("Invalid dangling pointer");
                }
                ptr
            },
            PhantomData,
        );

        Argument { meta, value, flag }
    }
}

impl<'a> Argument<'a> {
    #[inline]
    pub fn from_ref<T: 'static>(value: &'a T, flag: ArgumentFlag) -> Self {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Scalar,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: 1,
        };
        let value = ArgumentValue::Ref(ptr::NonNull::from(value).cast(), PhantomData);

        Self { meta, value, flag }
    }

    #[inline]
    pub fn from_mut<T: 'static>(value: &'a mut T, flag: ArgumentFlag) -> Self {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Scalar,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: 1,
        };
        let value = ArgumentValue::Mut(ptr::NonNull::from(value).cast(), PhantomData);

        Self { meta, value, flag }
    }

    #[inline]
    pub fn from_slice<T: 'static>(value: &'a [T], flag: ArgumentFlag) -> Self {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Slice,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: value.len(),
        };
        let ptr =
            ptr::NonNull::new(value.as_ptr().cast_mut().cast()).unwrap_or(ptr::NonNull::dangling());
        let value = ArgumentValue::Ref(ptr, PhantomData);

        Self { meta, value, flag }
    }

    #[inline]
    pub fn from_mut_slice<T: 'static>(value: &'a mut [T], flag: ArgumentFlag) -> Self {
        let meta = ArgumentMetadata {
            kind: ArgumentKind::Slice,
            type_id: TypeId::of::<T>(),
            type_size: size_of::<T>(),
            type_align: align_of::<T>(),
            len: value.len(),
        };
        let ptr = ptr::NonNull::new(value.as_mut_ptr().cast()).unwrap_or(ptr::NonNull::dangling());
        let value = ArgumentValue::Mut(ptr, PhantomData);

        Self { meta, value, flag }
    }
}

impl Argument<'_> {
    fn validate_metadata<T: 'static>(
        &self,
        expected_kind: ArgumentKind,
    ) -> Result<(), MessageError> {
        if self.meta.type_id != TypeId::of::<T>() {
            return Err(MessageError::ArgumentTypeMismatch);
        }

        match (self.meta.kind, expected_kind) {
            (ArgumentKind::Scalar, ArgumentKind::Slice) => {
                return Err(MessageError::ArgumentIsNotSlice);
            }
            (ArgumentKind::Slice, ArgumentKind::Scalar) => {
                return Err(MessageError::ArgumentIsNotScalar);
            }
            _ => {}
        }

        if size_of::<T>() > 0 {
            if self.meta.type_size != size_of::<T>() {
                return Err(MessageError::ArgumentTypeSizeMismatch {
                    expect: self.meta.type_size,
                    actual: size_of::<T>(),
                });
            }
            if self.meta.type_align != align_of::<T>() {
                return Err(MessageError::ArgumentTypeAlignMismatch {
                    expect: self.meta.type_align,
                    actual: align_of::<T>(),
                });
            }
        }

        Ok(())
    }

    #[inline]
    fn inner_val_ptr<T: Copy>(&self) -> Result<ptr::NonNull<T>, MessageError> {
        let ptr = match &self.value {
            ArgumentValue::Val(data) => ptr::NonNull::new(data.0.as_ptr().cast_mut())
                .expect("Inlined data pointer should not be NULL")
                .cast(),
            ArgumentValue::Ref(ptr, _) => ptr.cast(),
            ArgumentValue::Mut(ptr, _) => ptr.cast(),
        };

        if !ptr.is_aligned() {
            return Err(MessageError::UnalignedAccess);
        }

        Ok(ptr)
    }

    #[inline]
    fn inner_ref_ptr<T>(&self) -> Result<ptr::NonNull<T>, MessageError> {
        let ptr = match &self.value {
            ArgumentValue::Val(_) => return Err(MessageError::IllegalBorrowOfInlined),
            ArgumentValue::Ref(ptr, _) => ptr.cast::<T>(),
            ArgumentValue::Mut(ptr, _) => ptr.cast::<T>(),
        };

        if !ptr.is_aligned() {
            return Err(MessageError::UnalignedAccess);
        }

        Ok(ptr)
    }

    #[inline]
    fn inner_mut_ptr<T>(&self) -> Result<ptr::NonNull<T>, MessageError> {
        let ptr = match &self.value {
            ArgumentValue::Val(_) => return Err(MessageError::IllegalBorrowOfInlined),
            ArgumentValue::Ref(_, _) => return Err(MessageError::IllegalMutation),
            ArgumentValue::Mut(ptr, _) => ptr.cast::<T>(),
        };

        if !ptr.is_aligned() {
            return Err(MessageError::UnalignedAccess);
        }

        Ok(ptr)
    }
}

impl<'a> Argument<'a> {
    /// Attempts to downcast the argument to a value of type `T` by copying.
    ///
    /// This function succeeds if the argument's type matches `T` and the data
    /// is properly aligned. It works for both inlined values (`Val`) and
    /// referenced values (`Ref`, `Mut`).
    #[inline]
    pub fn downcast<T: Copy + 'static>(&self) -> Result<T, MessageError> {
        self.validate_metadata::<T>(ArgumentKind::Scalar)?;

        let ptr = self.inner_val_ptr()?;

        // SAFETY:
        // 1. `validate_metadata` ensures the `TypeId`, size, and align match `T`.
        // 2. `get_ptr` provides a valid pointer to the start of the data,
        //    whether it's inlined (`Val`) or external (`Ref`/`Mut`).
        // 3. We have confirmed the pointer is aligned for `T`.
        // 4. `T: Copy`, so reading the bytes to create a new value is safe.
        // Therefore, dereferencing the pointer to perform a copy is safe.
        Ok(*unsafe { ptr.as_ref() })
    }

    /// Attempts to downcast the argument to a reference of type `&'a T`.
    ///
    /// This will fail if the argument's value is stored inline (`ArgumentValue::Val`),
    /// because the lifetime of inlined data is tied to the `Argument` struct itself,
    /// not the longer lifetime `'a`.
    #[inline]
    pub fn downcast_ref<T: 'static>(&self) -> Result<&'a T, MessageError> {
        self.validate_metadata::<T>(ArgumentKind::Scalar)?;

        let ptr = self.inner_ref_ptr()?;

        // SAFETY:
        // The constructor functions (`from_ref`, `from_ptr`, etc.) guarantee that
        // for `Ref` and `Mut` variants, the pointer is valid for reads for the
        // entire lifetime 'a. We have explicitly checked and returned an error
        // for the `Val` variant, ensuring this code path is only taken for
        // pointer-based arguments whose data lives for 'a.
        Ok(unsafe { ptr.as_ref() })
    }

    /// Attempts to downcast the argument to a mutable reference of type `&'a mut T`.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can create a mutable reference `&'a mut T`
    /// from an internal raw pointer, bypassing the borrow checker.
    ///
    /// The caller MUST ensure that for the entire lifetime `'a` of the returned
    /// mutable reference, **NO other references (mutable or shared)** to the same
    /// data exist. This is a strict requirement of Rust's aliasing model.
    ///
    /// This includes any shared references that may have been previously created by
    /// calling methods like `downcast_ref` on this **same** `Argument` instance.
    ///
    /// Violating this requirement is immediate **undefined behavior**.
    #[inline]
    pub unsafe fn downcast_mut<T: 'static>(&self) -> Result<&'a mut T, MessageError> {
        self.validate_metadata::<T>(ArgumentKind::Scalar)?;

        let mut ptr = self.inner_mut_ptr()?;

        // SAFETY: The caller must uphold the safety contract described above.
        // We have also checked that the value is not inlined, so the pointer
        // corresponds to data with lifetime 'a, and `get_mut_ptr` ensures
        // the original argument was created from a mutable source.
        Ok(unsafe { ptr.as_mut() })
    }

    /// Attempts to downcast the argument to a slice of type `&'a [T]`.
    #[inline]
    pub fn downcast_slice<T: 'static>(&self) -> Result<&'a [T], MessageError> {
        self.validate_metadata::<T>(ArgumentKind::Slice)?;

        let ptr = self.inner_ref_ptr()?;

        // SAFETY:
        // The `from_slice` constructor ensures the pointer is valid for `len` elements
        // for the entire lifetime 'a. We have also ensured that this is not an
        // inlined value, so the lifetime 'a is appropriate.
        Ok(unsafe { slice::from_raw_parts(ptr.as_ptr(), self.meta.len) })
    }

    /// Attempts to downcast the argument to a mutable slice of type `&'a mut [T]`.
    ///
    /// # Safety
    ///
    /// This function is `unsafe` because it can create a mutable slice `&'a mut [T]`
    /// from an internal raw pointer, bypassing the borrow checker.
    ///
    /// The caller MUST ensure that for the entire lifetime `'a` of the returned
    /// mutable slice, **NO other references (mutable or shared)** to the same
    /// data exist. This is a strict requirement of Rust's aliasing model.
    ///
    /// This includes any shared references that may have been previously created by
    /// calling methods like `downcast_slice` on this **same** `Argument` instance.
    ///
    /// Violating this requirement is immediate **undefined behavior**.
    #[inline]
    pub unsafe fn downcast_mut_slice<T: 'static>(&self) -> Result<&'a mut [T], MessageError> {
        self.validate_metadata::<T>(ArgumentKind::Slice)?;

        let ptr = self.inner_mut_ptr()?;

        // SAFETY: The caller must uphold the safety contract described above.
        // The `from_mut_slice` constructor ensures the pointer is valid for reads
        // and writes for `len` elements for the entire lifetime 'a. `get_mut_ptr`
        // ensures the source was mutable.
        Ok(unsafe { slice::from_raw_parts_mut(ptr.as_ptr(), self.meta.len) })
    }
}

impl Argument<'_> {
    pub fn update_from(&mut self, source: &Argument<'_>) -> Result<(), MessageError> {
        if self.meta.type_id != source.meta.type_id {
            return Err(MessageError::ArgumentTypeMismatch);
        }

        if self.meta.type_size != source.meta.type_size {
            return Err(MessageError::ArgumentTypeSizeMismatch {
                expect: self.meta.type_size,
                actual: source.meta.type_size,
            });
        }

        if self.meta.type_align != source.meta.type_align {
            return Err(MessageError::ArgumentTypeAlignMismatch {
                expect: self.meta.type_align,
                actual: source.meta.type_align,
            });
        }

        if self.meta.len != source.meta.len {
            return Err(MessageError::ArgumentTypeLengthMismatch {
                expect: self.meta.len,
                actual: source.meta.len,
            });
        }

        let total_size = self.total_size();
        if total_size == 0 {
            return Ok(());
        }

        let (src_ptr, dst_ptr) = match (&source.value, &mut self.value) {
            (ArgumentValue::Val(src_bytes), ArgumentValue::Val(dst_bytes)) => {
                assert!(total_size <= INLINED_DATA_SIZE);
                (src_bytes.0.as_ptr(), dst_bytes.0.as_mut_ptr())
            }
            (ArgumentValue::Ref(src_ptr, _), ArgumentValue::Mut(dst_ptr, _))
            | (ArgumentValue::Mut(src_ptr, _), ArgumentValue::Mut(dst_ptr, _)) => {
                (src_ptr.as_ptr().cast_const(), dst_ptr.as_ptr())
            }
            (_, ArgumentValue::Ref(..)) => {
                return Err(MessageError::IllegalMutation);
            }
            _ => {
                return Err(MessageError::ArgumentStorageMismatch);
            }
        };
        unsafe {
            ptr::copy_nonoverlapping(src_ptr, dst_ptr, total_size);
        }

        Ok(())
    }
}

impl BytewiseReadOwned for Argument<'_> {
    fn read_from<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read argument
        let mut argument = unsafe { *reader.read_ref::<Argument>()? };

        if matches!(argument.value, ArgumentValue::Val(_)) {
            return Ok(argument);
        }

        // Read argument value
        // TODO: This `read_raw` call requires argument value impl `Copy` trait
        let data_ptr =
            unsafe { reader.read_raw(argument.meta.type_size, argument.meta.type_align)? };
        let value = ArgumentValue::Ref(data_ptr, PhantomData);

        // Update argument value
        argument.value = value;

        Ok(argument)
    }

    fn read_from_mut<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read argument metadata
        let mut argument = unsafe { *reader.read_ref::<Argument>()? };

        // Inlined data was already read from the buffer
        if matches!(argument.value, ArgumentValue::Val(_)) {
            return Ok(argument);
        }

        // Read argument value
        let data_ptr = unsafe {
            // TODO: `read_raw` call requires argument value impl `Copy` trait
            reader.read_raw(argument.meta.type_size, argument.meta.type_align)?
        };
        let value = ArgumentValue::Mut(data_ptr, PhantomData);

        // Update argument value
        argument.value = value;

        Ok(argument)
    }
}

impl BytewiseWrite for Argument<'_> {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        // Write argument metadata
        writer.write_ref(self)?;

        // Write argument value
        match self.value {
            ArgumentValue::Val(_) => {
                // Inlined data was already written to the buffer
            }
            ArgumentValue::Ref(ptr, _) => unsafe {
                // TODO: `write_raw` requires argument value impl `Copy` trait
                writer.write_raw(ptr, self.total_size(), self.type_align())?;
            },
            ArgumentValue::Mut(ptr, _) => unsafe {
                // TODO: `write_raw` requires argument value impl `Copy` trait
                writer.write_raw(ptr, self.total_size(), self.type_align())?;
            },
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct ZeroSizedStruct;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[allow(dead_code)]
    #[repr(align(16))]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct AlignedData(u32);

    #[test]
    fn test_empty_downcast() {
        let argument = Argument::empty();
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<()>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_empty_downcast_ref() {
        let argument = Argument::empty();
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<()>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(&()));
    }

    #[test]
    fn test_empty_downcast_mut() {
        let argument = Argument::empty();
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<()>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(&mut ()));
    }

    #[test]
    fn test_zst_from_value_downcast() {
        let orig_value = ZeroSizedStruct;
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<ZeroSizedStruct>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(orig_value));
    }

    #[test]
    fn test_zst_from_value_downcast_ref() {
        let orig_value = ZeroSizedStruct;
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<ZeroSizedStruct>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(&orig_value));
    }

    #[test]
    fn test_zst_from_value_downcast_mut() {
        let mut orig_value = ZeroSizedStruct;
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<ZeroSizedStruct>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(&mut orig_value));
    }

    #[test]
    fn test_from_value_downcast() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(orig_value));
    }

    #[test]
    fn test_from_value_downcast_ref() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::IllegalBorrowOfInlined));
    }

    #[test]
    fn test_from_value_downcast_mut() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<Point>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::IllegalBorrowOfInlined));
    }

    #[test]
    fn test_from_value_downcast_slice() {
        let orig_value = 1;
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_slice::<i32>();
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_value_downcast_mut_slice() {
        let orig_value = 1;
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_value(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut_slice::<i32>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_ref_downcast() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_ref(&orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(orig_value));
    }

    #[test]
    fn test_from_ref_downcast_ref() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_ref(&orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(&orig_value));
    }

    #[test]
    fn test_from_ref_downcast_mut() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_ref(&orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<Point>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::IllegalMutation));
    }

    #[test]
    fn test_from_ref_downcast_slice() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_ref(&orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_slice::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_ref_downcast_mut_slice() {
        let orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_ref(&orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut_slice::<Point>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_mut_downcast() {
        let mut orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut(&mut orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(orig_value));
    }

    #[test]
    fn test_from_mut_downcast_ref() {
        let mut orig_value = Point { x: 10, y: 20 };
        let value_ref = unsafe { &*ptr::from_ref(&orig_value) };

        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut(&mut orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(value_ref));
    }

    #[test]
    fn test_from_mut_downcast_mut() {
        let mut orig_value = Point { x: 10, y: 20 };

        let value_mut = unsafe { &mut *ptr::from_mut(&mut orig_value) };

        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut(&mut orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<Point>() };
        println!("result:    {:?}", result);

        let modified = result.expect("downcast_mut failed");
        modified.x = 100;
        modified.y = 100;

        println!("original:  {:?}", value_mut);
        println!("modified:  {:?}", modified);

        assert_eq!(modified, value_mut);
    }

    #[test]
    fn test_from_mut_downcast_slice() {
        let mut orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut(&mut orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_slice::<Point>();
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_mut_downcast_mut_slice() {
        let mut orig_value = Point { x: 10, y: 20 };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut(&mut orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut_slice::<Point>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::ArgumentIsNotSlice));
    }

    #[test]
    fn test_from_slice_downcast_ref() {
        let orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_slice(orig_value.as_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<i32>();
        println!("result:    {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentIsNotScalar)));
    }

    #[test]
    fn test_from_slice_downcast_mut() {
        let orig_value = [1, 2, 3, 4].as_slice();
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_slice(orig_value, ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<i32>() };
        println!("result:    {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentIsNotScalar)));
    }

    #[test]
    fn test_from_slice_downcast_slice() {
        let orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_slice(orig_value.as_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_slice::<i32>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(orig_value.as_slice()));
    }

    #[test]
    fn test_from_slice_downcast_mut_slice() {
        let orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_slice(orig_value.as_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut_slice::<i32>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Err(MessageError::IllegalMutation));
    }

    #[test]
    fn test_from_mut_slice_downcast() {
        let mut orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut_slice(orig_value.as_mut_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast::<&[i32]>();
        println!("result:    {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentTypeMismatch)));
    }

    #[test]
    fn test_from_mut_slice_downcast_ref() {
        let mut orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut_slice(orig_value.as_mut_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_ref::<i32>();
        println!("result:    {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentIsNotScalar)));
    }

    #[test]
    fn test_from_mut_slice_downcast_mut() {
        let mut orig_value = [1, 2, 3, 4];
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut_slice(orig_value.as_mut_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut::<i32>() };
        println!("result:    {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentIsNotScalar)));
    }

    #[test]
    fn test_from_mut_slice_downcast_slice() {
        let mut orig_value = [1, 2, 3, 4];
        let value_slice = unsafe { &*ptr::from_ref(orig_value.as_slice()) };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut_slice(orig_value.as_mut_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = argument.downcast_slice::<i32>();
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(value_slice));
    }

    #[test]
    fn test_from_mut_slice_downcast_mut_slice() {
        let mut orig_value = [1, 2, 3, 4];
        let value_slice = unsafe { &mut *ptr::from_mut(orig_value.as_mut_slice()) };
        println!("value:     {:?}", orig_value);

        let argument = Argument::from_mut_slice(orig_value.as_mut_slice(), ArgumentFlag::default());
        println!("argument:  {:#?}", argument);

        let result = unsafe { argument.downcast_mut_slice::<i32>() };
        println!("result:    {:?}", result);

        assert_eq!(result, Ok(value_slice));
    }

    #[test]
    fn test_type_mismatch() {
        let value = AlignedData(100);
        println!("value:    {:?}", value);

        let argument = Argument::from_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:#?}", argument);

        let result = argument.downcast_ref::<u32>();
        println!("result:   {:?}", result);

        assert!(matches!(result, Err(MessageError::ArgumentTypeMismatch)));
    }

    #[test]
    fn test_size_mismatch() {
        let value = AlignedData(100);
        println!("value:    {:?}", value);

        let mut argument = Argument::from_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:#?}", argument);

        argument.meta.type_size = 8;

        let result = argument.downcast_ref::<AlignedData>();
        println!("result:   {:?}", result);

        assert!(matches!(
            result,
            Err(MessageError::ArgumentTypeSizeMismatch {
                expect: _,
                actual: _
            })
        ));
    }

    #[test]
    fn test_align_mismatch() {
        let value = AlignedData(100);
        println!("value:    {:?}", value);

        let mut argument = Argument::from_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:#?}", argument);

        argument.meta.type_align = 8;

        let result = argument.downcast_ref::<AlignedData>();
        println!("result:   {:?}", result);

        assert!(matches!(
            result,
            Err(MessageError::ArgumentTypeAlignMismatch {
                expect: _,
                actual: _
            })
        ));
    }

    #[test]
    fn test_val_update_from() {
        let src_value: i32 = -1;
        let dst_value: i32 = 0;

        let src_arg = Argument::from_value(src_value, ArgumentFlag::default());
        let mut dst_arg = Argument::from_value(dst_value, ArgumentFlag::default());

        dst_arg
            .update_from(&src_arg)
            .expect("Update operation should succeed");

        let updated_value = dst_arg.downcast::<i32>().expect("Data type mismatched");
        println!("Before update: dst_value = {:?}", dst_value);
        println!("After update:  dst_value = {:?}", updated_value);

        assert_eq!(updated_value, -1, "Data was not copied correctly");
    }

    #[test]
    fn test_mut_update_from() {
        let src_data: [u32; 2] = [12345, 67890];
        let mut dst_data: [u32; 2] = [0, 0];
        println!("Before update: dst_data = {:?}", dst_data);

        let src_arg = Argument::from_slice(&src_data, ArgumentFlag::default());
        let mut dst_arg = Argument::from_mut_slice(&mut dst_data, ArgumentFlag::default());

        dst_arg
            .update_from(&src_arg)
            .expect("Update operation should succeed");

        println!("After update:  dst_data = {:?}", dst_data);
        assert_eq!(dst_data, [12345, 67890], "Data was not copied correctly");
    }

    #[test]
    fn test_ref_update_from() {
        let src_data: [u32; 2] = [12345, 67890];
        let dst_data: [u32; 2] = [99, 99];

        let src_arg = Argument::from_slice(&src_data, ArgumentFlag::default());
        let mut dst_arg = Argument::from_slice(&dst_data, ArgumentFlag::default());

        let result = dst_arg.update_from(&src_arg);
        assert_eq!(
            result,
            Err(MessageError::IllegalMutation),
            "Update should fail with IllegalMutation for a Ref destination"
        );
        println!("Update failed as expected: {:?}", result.unwrap_err());
        assert_eq!(dst_data, [99, 99], "Data should not be modified on failure");
    }
}
