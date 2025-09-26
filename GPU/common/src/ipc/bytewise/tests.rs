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

use std::fmt::Debug;

use super::*;

#[derive(Debug, PartialEq, Clone, Copy)]
struct TestStruct {
    a: i32,
    b: i8,
    c: [u16; 3],
}

fn test_write<T: Debug, W: BytewiseWriter>(writer: &mut W, value: &T) {
    let old_len = writer.written_bytes();

    unsafe { writer.write_ref(value).unwrap() };

    let new_len = writer.written_bytes();
    let write_len = new_len - old_len;

    println!(
        "write: {:2}, padding: {:2}, size: {:2}, align: {:2}, value: {:?}, type: {}",
        write_len,
        write_len - std::mem::size_of::<T>(),
        std::mem::size_of::<T>(),
        std::mem::align_of::<T>(),
        value,
        std::any::type_name::<T>()
    );
}

fn test_read<'a, T: Debug + PartialEq + 'a, R: BytewiseReader<'a>>(reader: &mut R, orig_value: &T) {
    let old_len = reader.read_bytes();
    let read_value = unsafe { reader.read_ref().unwrap() };
    let new_len = reader.read_bytes();
    let read_len = new_len - old_len;

    println!(
        "read: {:2}, padding: {:2}, size: {:2}, align: {:2}, value: {:?}, type: {}",
        read_len,
        read_len - std::mem::size_of::<T>(),
        std::mem::size_of::<T>(),
        std::mem::align_of::<T>(),
        read_value,
        std::any::type_name::<T>()
    );

    assert_eq!(orig_value, read_value);
}

fn test_read_mut<'a, T: Debug + PartialEq + 'a, R: BytewiseReader<'a>>(
    reader: &mut R,
    orig_value: &T,
) {
    let old_len = reader.read_bytes();
    let read_value = unsafe { reader.read_mut().unwrap() };
    let new_len = reader.read_bytes();
    let read_len = new_len - old_len;

    println!(
        "read: {:2}, padding: {:2}, size: {:2}, align: {:2}, value: {:?}, type: {}",
        read_len,
        read_len - std::mem::size_of::<T>(),
        std::mem::size_of::<T>(),
        std::mem::align_of::<T>(),
        read_value,
        std::any::type_name::<T>()
    );

    assert_eq!(orig_value, read_value);
}

#[test]
fn btyewise_read_write() {
    let test_zst: () = ();
    let test_bool: bool = true;
    let test_u8: u8 = 0xFF;
    let test_u16: u16 = 0b1010101001010101;
    let test_u32: u32 = 0x12345678;
    let test_f64: f64 = 3.141592653589793;
    let test_struct: TestStruct = TestStruct {
        a: 42,
        b: -1,
        c: [1, 2, 3],
    };
    let test_slice: [i32; 4] = [0xAA, 0xBB, 0xCC, 0xDD];
    let test_pointer: *const _ = std::ptr::from_ref(&test_struct);

    let mut buf = vec![0u8; 512];

    // Test BitwiseWrite
    let mut writer = BytewiseBuffer::new(&mut buf);
    test_write(&mut writer, &255u64);
    test_write(&mut writer, &test_zst);
    test_write(&mut writer, &test_bool);
    test_write(&mut writer, &test_pointer);
    test_write(&mut writer, &test_u32);
    test_write(&mut writer, &test_u8);
    test_write(&mut writer, &test_f64);
    test_write(&mut writer, &test_u16);
    test_write(&mut writer, &test_struct);
    test_write(&mut writer, &test_slice);

    let write_bytes = writer.written_bytes();
    println!("write {} bytes", write_bytes);
    println!("");

    println!("buf: {:?}", &buf[..write_bytes]);

    // Test BitwiseRead
    let mut reader = BytewiseBuffer::new(&buf);
    test_read(&mut reader, &255u64);
    test_read(&mut reader, &test_zst);
    test_read(&mut reader, &test_bool);
    test_read(&mut reader, &test_pointer);
    test_read(&mut reader, &test_u32);
    test_read(&mut reader, &test_u8);
    test_read(&mut reader, &test_f64);
    test_read(&mut reader, &test_u16);
    test_read(&mut reader, &test_struct);
    test_read(&mut reader, &test_slice);

    let read_bytes = reader.read_bytes();
    println!("read {} bytes", read_bytes);
    println!("");

    let mut reader = BytewiseBuffer::new(buf.as_mut_slice());
    test_read_mut(&mut reader, &255u64);
    test_read_mut(&mut reader, &test_zst);
    test_read_mut(&mut reader, &test_bool);
    test_read_mut(&mut reader, &test_pointer);
    test_read_mut(&mut reader, &test_u32);
    test_read_mut(&mut reader, &test_u8);
    test_read_mut(&mut reader, &test_f64);
    test_read_mut(&mut reader, &test_u16);
    test_read_mut(&mut reader, &test_struct);
    test_read_mut(&mut reader, &test_slice);

    let read_bytes = reader.read_bytes();
    println!("read {} bytes", read_bytes);

    assert_eq!(read_bytes, write_bytes);
}
