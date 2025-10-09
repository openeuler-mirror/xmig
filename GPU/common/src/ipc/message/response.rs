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

use crate::ipc::bytewise::{
    BytewiseError, BytewiseRead, BytewiseReadOwned, BytewiseReader, BytewiseWrite, BytewiseWriter,
};

use super::{Argument, ArgumentFlag, Request};

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct ResponseMetadata {
    request_id: u64,
    method_id: u64,
    arg_count: usize,
}

impl BytewiseRead for ResponseMetadata {
    fn read_ref<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<&'a Self, BytewiseError> {
        unsafe { reader.read_ref() }
    }
}

impl BytewiseWrite for ResponseMetadata {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        writer.write_ref(self)
    }
}

#[derive(Debug, Clone)]
pub struct Response<'a> {
    pub(super) request_id: u64,
    pub(super) method_id: u64,
    pub(super) arg_list: Vec<Argument<'a>>,
    pub(super) ret_value: Argument<'a>,
}

impl<'a> Response<'a> {
    #[inline]
    pub fn empty(request_id: u64, method_id: u64) -> Self {
        Self {
            request_id,
            method_id,
            arg_list: vec![],
            ret_value: Argument::empty(),
        }
    }

    #[inline]
    pub fn with_request<'b: 'a>(request: &Request<'a>, ret_value: Argument<'b>) -> Self {
        let request_id = request.request_id;
        let method_id = request.method_id;
        let arg_list = request
            .arg_list
            .iter()
            .map(|argument| {
                // Replace non-out argument to empty to save memory, but keep it's index
                if argument.flag().contains(ArgumentFlag::ARG_OUT) {
                    *argument
                } else {
                    Argument::empty()
                }
            })
            .collect();

        Self {
            request_id,
            method_id,
            ret_value,
            arg_list,
        }
    }

    #[inline]
    pub const fn request_id(&self) -> u64 {
        self.request_id
    }

    #[inline]
    pub const fn method_id(&self) -> u64 {
        self.method_id
    }

    #[inline]
    pub const fn argc(&self) -> usize {
        self.arg_list.len()
    }

    #[inline]
    pub const fn args(&self) -> &[Argument<'a>] {
        self.arg_list.as_slice()
    }

    #[inline]
    pub const fn args_mut(&mut self) -> &mut [Argument<'a>] {
        self.arg_list.as_mut_slice()
    }

    #[inline]
    pub const fn ret_value(&self) -> &Argument<'a> {
        &self.ret_value
    }
}

impl BytewiseReadOwned for Response<'_> {
    fn read_from<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read metadata
        let metadata = ResponseMetadata::read_ref(reader)?;

        // Read argument list
        let mut arg_list = Vec::with_capacity(metadata.arg_count);
        for _ in 0..metadata.arg_count {
            arg_list.push(Argument::read_from(reader)?);
        }

        // Read return value
        let ret_value = Argument::read_from(reader)?;

        Ok(Self {
            request_id: metadata.request_id,
            method_id: metadata.method_id,
            arg_list,
            ret_value,
        })
    }

    fn read_from_mut<'a, R: BytewiseReader<'a>>(reader: &mut R) -> Result<Self, BytewiseError> {
        // Read metadata
        let metadata = ResponseMetadata::read_ref(reader)?;

        // Read argument list
        let mut arg_list = Vec::with_capacity(metadata.arg_count);
        for _ in 0..metadata.arg_count {
            arg_list.push(Argument::read_from_mut(reader)?);
        }

        // Read return value
        let ret_value = Argument::read_from_mut(reader)?;

        Ok(Self {
            request_id: metadata.request_id,
            method_id: metadata.method_id,
            arg_list,
            ret_value,
        })
    }
}

impl BytewiseWrite for Response<'_> {
    fn write_to<W: BytewiseWriter>(&self, writer: &mut W) -> Result<(), BytewiseError> {
        let metadata = ResponseMetadata {
            request_id: self.request_id,
            method_id: self.method_id,
            arg_count: self.arg_list.len(),
        };

        // Write metadata
        metadata.write_to(writer)?;

        // Write argument list
        for arg in &self.arg_list {
            arg.write_to(writer)?;
        }

        // Write return value
        self.ret_value.write_to(writer)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use crate::ipc::{bytewise::BytewiseBuffer, message::ArgumentFlag};

    use super::*;

    fn response_roundtrip_test(response: Response<'_>) {
        let mut buf = vec![0u8; 4096];
        let mut writer = BytewiseBuffer::new(&mut buf);

        let send_resp = &response;
        println!(
            "send_resp: request_id: {}, method_id: {}, argument_count: {}",
            send_resp.request_id(),
            send_resp.method_id(),
            send_resp.argc()
        );
        let _ = send_resp.write_to(&mut writer).unwrap();

        let mut reader = BytewiseBuffer::new(&buf);
        let recv_resp = Response::read_from(&mut reader).unwrap();
        println!(
            "recv_resp: request_id: {}, method_id: {}, argument_count: {}",
            recv_resp.request_id(),
            recv_resp.method_id(),
            recv_resp.argc()
        );

        let mut send_iter = send_resp.args().iter();
        let mut recv_iter = recv_resp.args().iter();
        let mut index = 0;
        while let (Some(send_arg), Some(recv_arg)) = (send_iter.next(), recv_iter.next()) {
            println!("send_args[{}]: {:?}", index, send_arg);
            println!("recv_args[{}]: {:?}", index, recv_arg);

            if !send_arg.flag().contains(ArgumentFlag::ARG_OUT) {
                assert!(recv_arg.is_empty());
            } else {
                // assert_eq!(send_arg, recv_arg);
            }
            index += 1;
        }

        // assert_eq!(send_iter.next(), None);
        // assert_eq!(recv_iter.next(), None);

        // assert_eq!(send_resp.ret_value(), recv_resp.ret_value());

        // assert_eq!(&recv_resp, send_resp);
    }

    #[test]
    fn empty_response_roundtrip() {
        response_roundtrip_test(Response::empty(1, 0x1234));
    }

    #[test]
    fn response_with_out_args_roundtrip() {
        let request = Request::with_args(
            0xABCD,
            vec![
                Argument::from_ref(&42u32, ArgumentFlag::ARG_OUT),
                Argument::from_ref(&"test", ArgumentFlag::ARG_OUT),
            ],
        );
        println!("request: {:#?}", request);

        let ret_value = Argument::from_ref(&true, ArgumentFlag::ARG_OUT);
        let response = Response::with_request(&request, ret_value);
        println!("response: {:#?}", response);

        response_roundtrip_test(Response::with_request(&request, ret_value));
    }

    #[test]
    fn response_with_mixed_args_roundtrip() {
        let request = Request::with_args(
            0xDEAD,
            vec![
                Argument::from_ref(&1u8, ArgumentFlag::ARG_IN),
                Argument::from_ref(&2i16, ArgumentFlag::ARG_OUT),
                Argument::from_ref(&4u64, ArgumentFlag::ARG_OUT),
            ],
        );
        println!("request: {:#?}", request);

        let response = Response::with_request(&request, Argument::empty());
        println!("response: {:#?}", response);

        response_roundtrip_test(response);
    }

    #[test]
    fn response_with_complex_types_roundtrip() {
        #[derive(Debug, PartialEq)]
        struct TestStruct {
            field1: u32,
            field2: f64,
        }

        let request = Request::with_args(
            0xBEEF,
            vec![
                Argument::from_ref(
                    &TestStruct {
                        field1: 42,
                        field2: 3.14,
                    },
                    ArgumentFlag::ARG_OUT,
                ),
                Argument::from_ref(&ptr::null::<u8>(), ArgumentFlag::ARG_OUT),
            ],
        );
        println!("request: {:#?}", request);

        let ret_value = Argument::from_ref(
            &TestStruct {
                field1: 100,
                field2: 1.618,
            },
            ArgumentFlag::ARG_OUT,
        );
        let response = Response::with_request(&request, ret_value);
        println!("response: {:#?}", response);

        response_roundtrip_test(response);
    }

    #[test]
    fn response_methods_test() {
        let response = Response::empty(1, 0xCAFE);
        assert_eq!(response.method_id(), 0xCAFE);
        assert_eq!(response.argc(), 0);
        assert!(response.args().is_empty());
        assert!(response.ret_value().is_empty());

        let mut response = Response::with_request(
            &Request::with_arg(0x123, Argument::from_ref(&10u32, ArgumentFlag::ARG_OUT)),
            Argument::from_ref(&20u32, ArgumentFlag::ARG_OUT),
        );
        assert_eq!(response.argc(), 1);
        assert!(!response.args().is_empty());
        assert!(!response.ret_value().is_empty());

        let args_mut = response.args_mut();
        args_mut[0] = Argument::from_ref(&30u32, ArgumentFlag::ARG_OUT);
        assert_eq!(args_mut[0].downcast::<u32>().unwrap(), 30);
    }
}
