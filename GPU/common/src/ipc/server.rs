use super::{
    bytewise::{BytewiseBuffer, BytewiseReadOwned, BytewiseWrite},
    codec::FrameCodec,
    error::IpcError,
    message::{Request, Response},
    transport::{Endpoint, ReadBuf, Transport, WriteBuf},
};

#[derive(Debug)]
pub struct Server<T: Transport> {
    endpoint: T::Endpoint,
    framer: FrameCodec,
}

impl<T: Transport> Server<T> {
    pub fn create(transport: &T, addr: &T::Address) -> Result<Self, IpcError<T>> {
        Ok(Self {
            endpoint: transport
                .create(addr)
                .map_err(|e| IpcError::TransportError(e))?,
            framer: FrameCodec::new(40960),
        })
    }

    pub fn send_message<B: BytewiseWrite>(&mut self, message: &B) -> Result<(), IpcError<T>> {
        let mut write_buf = self
            .endpoint
            .write_buf()
            .map_err(|e| IpcError::TransportError(e))?;

        let (header, payload) = self.framer.prepare_buffer(&mut write_buf)?;

        let mut writer = BytewiseBuffer::new(payload);
        message.write_to(&mut writer)?;

        let frame_size = self.framer.encode_frame(header, writer.into_inner())?;
        write_buf
            .submit(frame_size)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(())
    }

    pub fn receive_message<B: BytewiseReadOwned>(&mut self) -> Result<Option<B>, IpcError<T>> {
        let read_buf = self
            .endpoint
            .read_buf()
            .map_err(|e| IpcError::TransportError(e))?;

        let (data_len, payload) = match self.framer.decode_frame(&read_buf)? {
            Some(result) => result,
            None => return Ok(None),
        };

        let mut reader = BytewiseBuffer::new(payload);
        let message = B::read_from_mut(&mut reader)?;

        read_buf
            .consume(data_len)
            .map_err(|e| IpcError::TransportError(e))?;

        Ok(Some(message))
    }

    pub fn invoke(&mut self, request: &Request) -> Result<Response<'_>, IpcError<T>> {
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
