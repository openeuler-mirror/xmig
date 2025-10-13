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

use std::ops::{Deref, DerefMut};

use super::{
    bytewise::{BytewiseBuffer, BytewiseReadOwned, BytewiseWrite, BytewiseWriter},
    error::IpcError,
    framer::{Frame, FrameBuf, Framer},
    message::{Request, Response},
    transport::{Endpoint, ReadBuf, Transport, WriteBuf},
};

#[derive(Debug)]
pub struct Peer<F: Framer, T: Transport> {
    framer: F,
    endpoint: T::Endpoint,
}

impl<F: Framer, T: Transport> Peer<F, T> {
    #[inline]
    pub fn new(framer: F, endpoint: T::Endpoint) -> Self {
        Self { framer, endpoint }
    }

    pub fn send_message<B: BytewiseWrite>(&mut self, message: &B) -> Result<(), IpcError<F, T>> {
        let mut write_buf = self
            .endpoint
            .write()
            .map_err(|e| IpcError::TransportError(e))?;

        let mut frame_buf = self.framer.encode_frame(&mut write_buf);

        let mut writer = BytewiseBuffer::new(frame_buf.as_mut());
        message.write_to(&mut writer)?;

        let payload_len = writer.written_bytes();
        let frame_len = frame_buf
            .finalize(payload_len)
            .map_err(|e| IpcError::FramerError(e))?;

        write_buf
            .submit(frame_len)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(())
    }

    pub fn receive_message<B: BytewiseReadOwned>(&mut self) -> Result<Option<B>, IpcError<F, T>> {
        let read_buf = self
            .endpoint
            .read()
            .map_err(|e| IpcError::TransportError(e))?;

        let frame = match self
            .framer
            .decode_frame(&read_buf)
            .map_err(|e| IpcError::FramerError(e))?
        {
            Some(result) => result,
            None => return Ok(None),
        };

        let mut reader = BytewiseBuffer::new(frame.as_ref());
        let message = B::read_from_mut(&mut reader)?;

        let frame_len = frame.frame_len();
        drop(frame);

        read_buf
            .consume(frame_len)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(Some(message))
    }

    pub fn invoke(&mut self, request: &Request) -> Result<Response<'_>, IpcError<F, T>> {
        self.send_message(request)?;

        let response = loop {
            match self.receive_message()? {
                Some(resp) => break resp,
                None => continue,
            }
        };

        Ok(response)
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Server<F: Framer, T: Transport>(Peer<F, T>);

impl<F: Framer, T: Transport> Server<F, T> {
    #[inline]
    pub fn create(framer: F, transport: &T, addr: &T::Address) -> Result<Self, IpcError<F, T>> {
        let endpoint = transport
            .create(addr)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(Self(Peer::new(framer, endpoint)))
    }
}

impl<F: Framer, T: Transport> Deref for Server<F, T> {
    type Target = Peer<F, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Framer, T: Transport> DerefMut for Server<F, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[repr(transparent)]
#[derive(Debug)]
pub struct Client<F: Framer, T: Transport>(Peer<F, T>);

impl<F: Framer, T: Transport> Client<F, T> {
    #[inline]
    pub fn connect(framer: F, transport: &T, addr: &T::Address) -> Result<Self, IpcError<F, T>> {
        let endpoint = transport
            .connect(addr)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(Self(Peer::new(framer, endpoint)))
    }
}

impl<F: Framer, T: Transport> Deref for Client<F, T> {
    type Target = Peer<F, T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<F: Framer, T: Transport> DerefMut for Client<F, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
