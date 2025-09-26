use super::{
    bytewise::{BytewiseBuffer, BytewiseReadOwned, BytewiseWrite},
    codec::FrameCodec,
    error::IpcError,
    message::{Request, Response},
    transport::{Endpoint, ReadBuf, Transport, TransportError, WriteBuf},
};

#[derive(Debug)]
pub struct Server<T: Transport> {
    endpoint: T::Endpoint,
    framer: FrameCodec,
}

impl<T: Transport> Server<T> {
    pub fn create(transport: &T, addr: &T::Address) -> Result<Self, IpcError> {
        Ok(Self {
            endpoint: transport.create(addr)?,
            framer: FrameCodec::new(40960),
        })
    }

    pub fn send_message<B: BytewiseWrite>(&mut self, message: &B) -> Result<(), IpcError> {
        let mut write_buf = self.endpoint.write_buf()?;

        let (header, payload) = self.framer.prepare_buffer(&mut write_buf)?;

        let mut writer = BytewiseBuffer::new(payload);
        message.write_to(&mut writer)?;

        let frame_size = self.framer.encode_frame(header, writer.into_inner())?;
        write_buf.submit(frame_size)?;

        Ok(())
    }

    pub fn receive_message<B: BytewiseReadOwned>(&mut self) -> Result<Option<B>, IpcError> {
        let read_buf = self.endpoint.read_buf()?;

        let (data_len, payload) = match self.framer.decode_frame(&read_buf)? {
            Some(result) => result,
            None => return Ok(None),
        };

        let mut reader = BytewiseBuffer::new(payload);
        let message = B::read_from(&mut reader)?;

        read_buf.consume(data_len)?;

        Ok(Some(message))
    }

    pub fn invoke(&mut self, request: &Request) -> Result<Response<'_>, IpcError> {
        self.send_message(request)?;
        self.receive_message()?
            .ok_or(IpcError::TransportError(TransportError::ConnectionNotReady))
    }
}
