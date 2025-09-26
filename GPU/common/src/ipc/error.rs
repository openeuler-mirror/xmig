use thiserror::Error;

use super::{bytewise::BytewiseError, codec::FrameCodecError, transport::TransportError};

#[derive(Debug, Error)]
pub enum IpcError {
    #[error("Transport Error: {0}")]
    TransportError(#[from] TransportError),

    #[error("Codec Error: {0}")]
    CodecError(#[from] FrameCodecError),

    #[error("Bytewise Error: {0}")]
    BytewiseError(#[from] BytewiseError),
}
