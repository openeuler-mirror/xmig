use crate::ipc::{
    bytewise::{BytewiseBuffer, BytewiseReadOwned, BytewiseWrite, BytewiseWriter},
    framer::{Frame, FrameBuf, Framer},
    message::{Request, Response},
    transport::{Endpoint, ReadBuf, Transport, WriteBuf},
};

use super::error::IpcError;

#[derive(Debug)]
pub struct Server<C: Framer, T: Transport> {
    framer: C,
    endpoint: T::Endpoint,
}

impl<C: Framer, T: Transport> Server<C, T> {
    pub fn create(framer: C, transport: &T, addr: &T::Address) -> Result<Self, IpcError<C, T>> {
        let endpoint = transport
            .create(addr)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(Self { framer, endpoint })
    }

    pub fn send_message<B: BytewiseWrite>(&mut self, message: &B) -> Result<(), IpcError<C, T>> {
        let mut write_buf = self
            .endpoint
            .write_buf()
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

    pub fn receive_message<B: BytewiseReadOwned>(&mut self) -> Result<Option<B>, IpcError<C, T>> {
        let read_buf = self
            .endpoint
            .read_buf()
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

    pub fn invoke(&mut self, request: &Request) -> Result<Response<'_>, IpcError<C, T>> {
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
