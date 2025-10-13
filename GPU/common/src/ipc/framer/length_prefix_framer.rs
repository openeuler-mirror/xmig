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

use thiserror::Error;
use tracing::debug;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, NativeEndian, U32};

use super::{Frame, FrameBuf, Framer};

const FRAME_MAGIC_NUMBER: u32 = 0x78464D45; // 'xFME' in ASCII
pub const DEFAULT_MAX_FRAME_LEN: usize = 16 * 1024 * 1024;

#[derive(Debug, Error)]
pub enum LengthPrefixFramerError {
    #[error("insufficient buffer capacity (required: {required}, capacity: {capacity})")]
    InsufficientBuffer { required: usize, capacity: usize },

    #[error("frame header is not aligned (align: {align})")]
    UnalignedHeader { align: usize },

    #[error("frame header is invalid")]
    InvalidHeader,

    #[error("frame magic numver mismatch (expected: {expected:#010X}, actual: {actual:#010X})")]
    MagicNumberMismatch { expected: u32, actual: u32 },

    #[error("frame length exceeds limit (limit: {limit}, actual: {actual})")]
    FrameTooLarge { limit: usize, actual: usize },

    #[error("frame checksum mismatch (expected: {expected:#010X}, actual: {actual:#010X})")]
    ChecksumMismatch { expected: u32, actual: u32 },
}

#[repr(C)]
#[derive(Debug, FromBytes, IntoBytes, KnownLayout, Immutable)]
struct FrameHeader {
    magic: U32<NativeEndian>,
    length: U32<NativeEndian>,
    checksum: U32<NativeEndian>,
}

#[derive(Debug)]
struct LengthPrefixFrame<B> {
    payload: B,
}

impl<B: AsRef<[u8]>> Deref for LengthPrefixFrame<B> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        self.payload.as_ref()
    }
}

impl<B: AsRef<[u8]>> Frame for LengthPrefixFrame<B> {
    fn frame_len(&self) -> usize {
        size_of::<FrameHeader>().saturating_add(self.payload.as_ref().len())
    }
}

#[derive(Debug)]
struct LengthPrefixFrameBuffer<B> {
    buffer: B,
    limit: usize,
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> FrameBuf for LengthPrefixFrameBuffer<B> {
    type Error = LengthPrefixFramerError;

    fn finalize(mut self, payload_len: usize) -> Result<usize, Self::Error> {
        let buf_len = self.buffer.as_ref().len();
        let frame_len = size_of::<FrameHeader>() + payload_len;

        if frame_len > self.limit {
            return Err(LengthPrefixFramerError::FrameTooLarge {
                limit: self.limit,
                actual: frame_len,
            });
        }

        if frame_len > buf_len {
            return Err(LengthPrefixFramerError::InsufficientBuffer {
                required: frame_len,
                capacity: buf_len,
            });
        }

        let (header_buf, payload_buf) = self.buffer.as_mut().split_at_mut(size_of::<FrameHeader>());

        let mut header = FrameHeader {
            magic: U32::new(FRAME_MAGIC_NUMBER),
            length: U32::new(payload_len as u32),
            checksum: U32::new(0),
        };

        let checksum = {
            let mut crc32 = crc32fast::Hasher::new();
            crc32.update(header.length.as_bytes());
            crc32.update(&payload_buf[..payload_len]);
            crc32.finalize()
        };
        header.checksum.set(checksum);

        header
            .write_to_prefix(header_buf)
            .expect("Header buffer is correctly sized");

        debug!(
            "[Frame] Encoded {} bytes (payload {} bytes)",
            frame_len, payload_len
        );
        Ok(frame_len)
    }
}

impl<B: AsRef<[u8]>> Deref for LengthPrefixFrameBuffer<B> {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        &self.buffer.as_ref()[size_of::<FrameHeader>()..]
    }
}

impl<B: AsRef<[u8]> + AsMut<[u8]>> DerefMut for LengthPrefixFrameBuffer<B> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.buffer.as_mut()[size_of::<FrameHeader>()..]
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LengthPrefixFramer {
    limit: usize,
}

impl LengthPrefixFramer {
    #[inline]
    pub fn new(limit: usize) -> Self {
        assert!(limit >= size_of::<FrameHeader>());

        Self { limit }
    }
}

impl Default for LengthPrefixFramer {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_FRAME_LEN)
    }
}

impl Framer for LengthPrefixFramer {
    type Error = LengthPrefixFramerError;

    fn encode_frame<'a>(&self, buf: &'a mut [u8]) -> impl FrameBuf<Error = Self::Error> + 'a {
        LengthPrefixFrameBuffer {
            buffer: buf,
            limit: self.limit,
        }
    }

    fn decode_frame<'a>(&self, buf: &'a [u8]) -> Result<Option<impl Frame + 'a>, Self::Error> {
        let buf_len = buf.len();
        let header_len = size_of::<FrameHeader>();

        /* Skip if no enough data for a full header */
        if buf_len < header_len {
            return Ok(None);
        }

        /* Parse frame header */
        let header = FrameHeader::ref_from_bytes(&buf[..header_len]).map_err(|e| match e {
            zerocopy::ConvertError::Alignment(_) => LengthPrefixFramerError::UnalignedHeader {
                align: align_of::<FrameHeader>(),
            },
            zerocopy::ConvertError::Size(_) => LengthPrefixFramerError::InsufficientBuffer {
                required: header_len,
                capacity: buf_len,
            },
            zerocopy::ConvertError::Validity(_) => LengthPrefixFramerError::InvalidHeader,
        })?;

        /* Check frame magic number */
        let magic = header.magic.get();
        if magic != FRAME_MAGIC_NUMBER {
            return Err(LengthPrefixFramerError::MagicNumberMismatch {
                expected: FRAME_MAGIC_NUMBER,
                actual: magic,
            });
        }

        /* Check frame size */
        let payload_len = header.length.get() as usize;
        let frame_len = header_len + payload_len;
        if frame_len > self.limit {
            return Err(LengthPrefixFramerError::FrameTooLarge {
                limit: self.limit,
                actual: frame_len,
            });
        }

        /* Skip if no enough data for the whole frame */
        if buf_len < frame_len {
            return Ok(None);
        }

        /* Get frame payload */
        let payload = &buf[header_len..frame_len];

        /* Calculate frame checksum */
        let checksum = {
            let mut crc32 = crc32fast::Hasher::new();
            crc32.update(header.length.as_bytes());
            crc32.update(payload);
            crc32.finalize()
        };

        /* Check frame checksum */
        let header_checksum = header.checksum.get();
        if checksum != header_checksum {
            return Err(LengthPrefixFramerError::ChecksumMismatch {
                expected: header_checksum,
                actual: checksum,
            });
        }

        debug!(
            "[Frame] Decoded {} bytes (payload {} bytes)",
            frame_len, payload_len
        );
        Ok(Some(LengthPrefixFrame { payload }))
    }
}
