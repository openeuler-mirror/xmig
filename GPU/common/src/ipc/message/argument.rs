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

use std::{marker::PhantomData, ptr, slice};
use std::sync::Arc;
use std::any::Any;
use std::fmt;

use bitflags::bitflags;
use thiserror::Error;

use crate::ipc::bytewise::{
    BytewiseReader, BytewiseWriter, BytewiseError, BytewiseReadOwned, BytewiseWrite,
};

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

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum ArgumentError {
    #[error("Argument type size mismatch (expected: {expected}, found: {actual})")]
    TypeSizeMismatch { expected: usize, actual: usize },

    #[error("Argument type alignment mismatch (expected: {expected}, found: {actual})")]
    TypeAlignMismatch { expected: usize, actual: usize },

    #[error("Attempted to access null pointer")]
    NullPointer,

    #[error(
        "Attempted to access unaligned data (type_name: '{type_name}', alignment: {alignment})"
    )]
    UnalignedData {
        type_name: &'static str,
        alignment: usize,
    },

    #[error("Attempted to access non mutable data as mutable")]
    NonMutableData,

    #[error("Attempted to access Value as Pointer")]
    ExpectedPointer,

    #[error("Attempted to access Pointer as Value")]
    ExpectedValue,

    #[error("Attempted to access Value with mismatched type")]
    TypeMismatch,
}

#[repr(C)]
#[derive(Clone)]
pub enum Argument<'a> {
    Pointer {
        ptr: *const u8,
        mutable: bool,
        align: usize,
        size: usize,
        count: usize,
        flag: ArgumentFlag,
        data: PhantomData<&'a ()>,
    },
    Value {
        value: Arc<dyn Any + 'static>,
        flag: ArgumentFlag
    }
}

impl<'a> Argument<'a> {
    pub fn try_ptr<T>(&self) -> Result<*const T, ArgumentError> {
        match self {
            Argument::Pointer { ptr, size, align, .. } => {
                let expected_size = size_of::<T>();
                if *size != expected_size {
                    return Err(ArgumentError::TypeSizeMismatch {
                        expected: expected_size,
                        actual: *size,
                    });
                }

                let expected_align = align_of::<T>();
                if *align != expected_align {
                    return Err(ArgumentError::TypeAlignMismatch {
                        expected: expected_align,
                        actual: *align,
                    });
                }

                let data_ptr = *ptr as *const T;

                if data_ptr.is_null() {
                    return Err(ArgumentError::NullPointer);
                }

                if (data_ptr as usize) % align_of::<T>() != 0 {
                    return Err(ArgumentError::UnalignedData {
                        type_name: std::any::type_name::<T>(),
                        alignment: align_of::<T>(),
                    });
                }

                Ok(data_ptr)
            }
            Argument::Value { .. } => Err(ArgumentError::ExpectedPointer),
        }
    }

    pub fn try_mut_ptr<T>(&self) -> Result<*mut T, ArgumentError> {
        let const_ptr = self.try_ptr::<T>()?;

        // 再次 match 获取 mutable 标志
        match self {
            Argument::Pointer { mutable, .. } => {
                if !*mutable {
                    return Err(ArgumentError::NonMutableData);
                }
            }
            Argument::Value { .. } => return Err(ArgumentError::ExpectedPointer),
        }

        Ok(const_ptr as *mut T)
    }
}

impl<'a> Argument<'a> {
    #[inline]
    pub const fn empty() -> Self {
        // Create argument from stack ZST reference is safe
        Self::with_ref(&(), ArgumentFlag::ARG_IN)
    }

    #[inline]
    pub fn with_value<T: 'static>(value: T, flag: ArgumentFlag) -> Self {
        Self::Value { value: Arc::new(value), flag: (flag) }
    }

    #[inline]
    pub const fn with_ref<T>(value: &'a T, flag: ArgumentFlag) -> Self {
        Self::Pointer {
            ptr: ptr::from_ref(value).cast(),
            mutable: false,
            size: size_of::<T>(),
            align: align_of::<T>(),
            count: 1,
            flag,
            data: PhantomData,
        }
    }

    #[inline]
    pub const fn with_ref_mut<T>(value: &'a mut T, flag: ArgumentFlag) -> Self {
        Self::Pointer {
            ptr: ptr::from_mut(value).cast(),
            mutable: true,
            size: size_of::<T>(),
            align: align_of::<T>(),
            count: 1,
            flag,
            data: PhantomData,
        }
    }

    #[inline]
    pub const fn with_slice<T>(value: &'a [T], flag: ArgumentFlag) -> Self {
        Self::Pointer {
            ptr: value.as_ptr().cast(),
            mutable: false,
            size: size_of::<T>(),
            align: align_of::<T>(),
            count: value.len(),
            flag,
            data: PhantomData,
        }
    }

    #[inline]
    pub const fn with_slice_mut<T>(value: &'a mut [T], flag: ArgumentFlag) -> Self {
        Self::Pointer {
            ptr: value.as_mut_ptr().cast(),
            mutable: true,
            size: size_of::<T>(),
            align: align_of::<T>(),
            count: value.len(),
            flag,
            data: PhantomData,
        }
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        match self {
            Argument::Pointer { size, .. } => *size == 0,
            Argument::Value { .. } => false,
        }
    }

    #[inline]
    pub fn flag(&self) -> ArgumentFlag {
        match self {
            Argument::Pointer { flag, .. } => *flag,
            Argument::Value { flag, .. } => *flag,
        }
    }

    #[inline]
   pub fn try_value<T: Any + 'static + Clone>(&self) -> Result<T, ArgumentError> {
        match self {
            Argument::Pointer { .. } => Err(ArgumentError::ExpectedValue),
            Argument::Value { value, .. } => {
                if let Some(concrete) = value.downcast_ref::<T>() {
                    return Ok(concrete.clone());
                }
                if let Some(arc) = value.downcast_ref::<Arc<T>>() {
                    return Ok((**arc).clone());
                }
                Err(ArgumentError::TypeMismatch)
            }
        }
    }

    #[inline]
    pub fn try_ref<T>(&self) -> Result<&T, ArgumentError> {
        let value = unsafe { &*self.try_ptr::<T>()? };

        Ok(value)
    }

    #[inline]
    pub fn try_ref_mut<T>(&mut self) -> Result<&mut T, ArgumentError> {
        let value_mut = unsafe { &mut *self.try_mut_ptr::<T>()? };

        Ok(value_mut)
    }

    #[inline]
    pub fn try_slice<T>(&self) -> Result<&[T], ArgumentError> {
        let count = match self {
            Argument::Pointer { count, .. } => *count,
            Argument::Value { .. } => return Err(ArgumentError::ExpectedPointer),
        };

        let ptr = self.try_ptr::<T>()?;
        let slice = unsafe { slice::from_raw_parts(ptr, count) };
        //let slice = unsafe { slice::from_raw_parts(self.try_ptr::<T>()?, self.count) };

        Ok(slice)
    }

    #[inline]
    pub fn try_slice_mut<T>(&mut self) -> Result<&mut [T], ArgumentError> {
        let count = match self {
            Argument::Pointer { count, .. } => *count,
            Argument::Value { .. } => return Err(ArgumentError::ExpectedPointer),
        };
        let slice_mut = unsafe { slice::from_raw_parts_mut(self.try_mut_ptr::<T>()?, count) };

        Ok(slice_mut)
    }

    pub const fn size(&self) -> usize {
        match self {
            Argument::Pointer { size, .. } => *size,
            Argument::Value { .. } => {
                0
            }
        }
    }

    pub const fn align(&self) -> usize {
        match self {
            Argument::Pointer { align, .. } => *align,
            Argument::Value { .. } => 1,
        }
    }

    pub const fn count(&self) -> usize {
        match self {
            Argument::Pointer { count, .. } => *count,
            Argument::Value { .. } => 1,
        }
    }

    /// 是否为指针类型
    pub const fn is_pointer(&self) -> bool {
        matches!(self, Argument::Pointer { .. })
    }

    /// 是否为值类型
    pub const fn is_value(&self) -> bool {
        matches!(self, Argument::Value { .. })
    }
}


impl<'a> fmt::Debug for Argument<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Pointer { ptr, mutable, align, size, count, flag, .. } => f
                .debug_struct("Argument::Pointer")
                .field("ptr", ptr)
                .field("mutable", mutable)
                .field("align", align)
                .field("size", size)
                .field("count", count)
                .field("flag", flag)
                .finish(),
            Argument::Value { value, flag } => f
                .debug_struct("Argument::Value")
                .field("value_type", &std::any::type_name_of_val(value))
                .field("flag", flag)
                .finish(),
        }
    }
}


impl<'a> PartialEq for Argument<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (
                Argument::Pointer {
                    ptr: p1,
                    mutable: m1,
                    align: a1,
                    size: s1,
                    count: c1,
                    flag: f1,
                    ..
                },
                Argument::Pointer {
                    ptr: p2,
                    mutable: m2,
                    align: a2,
                    size: s2,
                    count: c2,
                    flag: f2,
                    ..
                },
            ) => {
                // 快速比较元数据
                if m1 != m2 || a1 != a2 || s1 != s2 || c1 != c2 || f1 != f2 {
                    return false;
                }

                // 零大小类型
                if *s1 == 0 {
                    return true;
                }

                // 指针相同
                if *p1 == *p2 {
                    return true;
                }

                // 任一为空
                if (*p1 as usize) == 0 || (*p2 as usize) == 0 {
                    return false;
                }

                // 比较底层字节
                let bytes1 = unsafe { slice::from_raw_parts(*p1, *s1) };
                let bytes2 = unsafe { slice::from_raw_parts(*p2, *s2) };
                bytes1 == bytes2
            }
            (Argument::Value { value: v1, flag: f1 }, Argument::Value { value: v2, flag: f2 }) => {

                let type_eq = v1.type_id() == v2.type_id();
                let flag_eq = f1 == f2;
                type_eq && flag_eq
            }
            _ => false,
        }
    }
}

impl<'a> Eq for Argument<'a> {}

impl<'a> BytewiseReadOwned for Argument<'a> {
    fn read_from<'b, R: BytewiseReader<'b>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // 读取 Argument 结构（不含实际数据）
        let arg_meta = unsafe { reader.read_ref::<Argument>()? };

        // 2. 根据元数据创建新的 Argument 实例
        let mut arg = match *arg_meta {
            Argument::Pointer {
                align,
                size,
                count,
                flag,
                ..
            } => Argument::Pointer {
                ptr: std::ptr::null(),     // 临时空指针
                mutable: false,
                align,
                size,
                count,
                flag,
                data: PhantomData,
            },
            Argument::Value { ref value, flag } =>
            Argument::Value { value: value.clone(), flag }
        };

        // 3. 读取实际数据到新指针
        let data_ptr = unsafe {
            reader.read_ptr(arg.size(), arg.align())?
        };

        // 4. 更新新实例的指针字段
        if let Argument::Pointer { ptr, mutable, data, .. } = &mut arg {
            *ptr = data_ptr;
            *mutable = false;
            *data = PhantomData;
        }

        Ok(arg)
    }

    fn read_from_mut<'b, R: BytewiseReader<'b>>(reader: &mut R) -> Result<Self, BytewiseError> {
        let arg_meta = unsafe { reader.read_ref::<Argument>()? };

        let mut arg = match arg_meta {
            Argument::Pointer {
                align, size, count, flag, ..
            } => {
                let data_ptr = unsafe { reader.read_ptr(*size, *align)? };
                Argument::Pointer {
                ptr: data_ptr,
                mutable: true,
                align: *align,
                size: *size,
                count: *count,
                flag: *flag,
                data: PhantomData,
            }
        },
            Argument::Value {value, flag} => Argument::Value { value: value.clone(), flag: *flag }
        };

        let data_ptr = unsafe {
            reader.read_ptr(arg.size(), arg.align())?
        };

        if let Argument::Pointer { ptr, mutable, data, .. } = &mut arg {
            *ptr = data_ptr;
            *mutable = true;
            *data = PhantomData;
        }

        Ok(arg)
    }
}

impl<'a> BytewiseWrite for Argument<'a> {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        // 写入 Argument 元数据
        unsafe {
            writer.write_ref(self)?;
        }

        // 写入指针指向的数据
        if let Argument::Pointer { ptr, size, align, .. } = self
            && *size > 0 && !ptr.is_null() {
                unsafe {
                    writer.write_raw_ptr(*ptr, *size, *align)?;
                }
            }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Eq)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[test]
    fn empty_value() {
        let argument = Argument::empty();
        println!("argument:  {:?}", argument);

        let value_ref = argument.try_ref::<()>().unwrap();
        println!("value:     {:?}", value_ref);

        assert_eq!(argument.flag(), ArgumentFlag::ARG_IN);
    }

    #[test]
    fn value_success() {
        let value = Point { x: 10, y: 20 };
        println!("value:    {:?}", value);

        let argument = Argument::with_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument:  {:?}", argument);

        let value_ref = argument.try_ref::<Point>().unwrap();
        println!("value:     {:?}", value);

        assert_eq!(*value_ref, value);
        assert_eq!(argument.flag(), ArgumentFlag::ARG_IN);
    }

    #[test]
    fn value_type_mismatch_size() {
        let value: i32 = 42;
        println!("value:    {:?}", value);

        let argument = Argument::with_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let result = argument.try_ref::<i64>();
        assert!(matches!(
            result,
            Err(ArgumentError::TypeSizeMismatch { expected, actual })
            if expected == size_of::<i64>() && actual == size_of::<i32>()
        ));
    }

    #[test]
    fn value_type_mismatch_align() {
        #[allow(dead_code)]
        #[repr(align(32))]
        #[derive(Debug)]
        struct AlignedData(u32);

        let value = AlignedData(100);
        println!("value:    {:?}", value);

        let argument = Argument::with_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let result = argument.try_ref::<[u32; 8]>();
        println!("result:   {:?}", result);

        assert!(matches!(
            result,
            Err(ArgumentError::TypeAlignMismatch { expected, actual })
            if expected == align_of::<u32>() && actual == 32
        ));
    }

    #[test]
    fn value_mut_success() {
        let mut value = Point { x: 10, y: 20 };
        println!("value:    {:?}", value);

        let mut argument = Argument::with_ref_mut(&mut value, ArgumentFlag::ARG_OUT);
        println!("argument: {:?}", argument);

        let value_mut_ref = argument.try_ref_mut::<Point>().unwrap();
        value_mut_ref.x = 99;

        assert_eq!(argument.flag(), ArgumentFlag::ARG_OUT);
        assert_eq!(value.x, 99);
    }

    #[test]
    fn value_mut_on_immutable_fails() {
        let value = Point { x: 10, y: 20 };
        println!("value:    {:?}", value);

        let mut argument = Argument::with_ref(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let result = argument.try_ref_mut::<Point>();
        println!("result:   {:?}", result);

        assert_eq!(result.unwrap_err(), ArgumentError::NonMutableData);
    }

    #[test]
    fn u16_slice_success() {
        let value = [1u16, 2, 3, 4, 5];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let slice_ref = argument.try_slice::<u16>().unwrap();
        assert_eq!(slice_ref, &value[..]);
    }

    #[test]
    fn i32_slice_success() {
        let value = [1i32, 2, 3, 4, 5];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let slice_ref = argument.try_slice::<i32>().unwrap();
        assert_eq!(slice_ref, &value[..]);
    }

    #[test]
    fn f64_slice_success() {
        let value = [1.1, 2.2, 3.3, 4.4, 5.5];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let slice_ref = argument.try_slice::<f64>().unwrap();
        assert_eq!(slice_ref, &value[..]);
    }

    #[test]
    fn slice_empty_success() {
        let value: &[i32] = &[];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let slice_ref = argument.try_slice::<i32>().unwrap();
        assert_eq!(slice_ref.len(), 0);
    }

    #[test]
    fn slice_type_mismatch() {
        let value = [1i32, 2, 3, 4, 5];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let result = argument.try_slice::<u8>();
        println!("result:   {:?}", result);

        assert!(matches!(
            result,
            Err(ArgumentError::TypeSizeMismatch { expected, .. })
            if expected == size_of::<u8>()
        ));
    }

    #[test]
    fn slice_mut_success() {
        let mut value = [1i32, 2, 3, 4, 5];
        println!("value:    {:?}", value);

        let mut argument = Argument::with_slice_mut(&mut value, ArgumentFlag::ARG_OUT);
        println!("argument: {:?}", argument);

        let slice_mut_ref = argument.try_slice_mut::<i32>().unwrap();
        slice_mut_ref[0] = 100;

        assert_eq!(value[0], 100);
    }

    #[test]
    fn slice_mut_on_immutable_fails() {
        let value = [1i32, 2, 3, 4, 5];
        println!("value:    {:?}", value);

        let mut argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let result = argument.try_slice_mut::<i32>();
        println!("result:   {:?}", result);

        assert_eq!(result.unwrap_err(), ArgumentError::NonMutableData);
    }

    #[test]
    fn access_slice_as_value() {
        let value = [Point { x: 1, y: 2 }, Point { x: 3, y: 4 }];
        println!("value:    {:?}", value);

        let argument = Argument::with_slice(&value, ArgumentFlag::ARG_IN);
        println!("argument: {:?}", argument);

        let value_ref = argument.try_ref::<Point>().unwrap();
        assert_eq!(*value_ref, value[0]);
    }
}
