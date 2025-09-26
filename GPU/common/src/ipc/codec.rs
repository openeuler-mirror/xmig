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

//! A robust binary framing protocol implementation.
//!
//! This module provides a `FrameCodec` type that implements a binary framing protocol with:
//! - Magic number identification
//! - Length-prefixed payloads
//! - CRC32 checksum verification

use std::mem;

use thiserror::Error;
use zerocopy::{FromBytes, FromZeros, Immutable, IntoBytes, KnownLayout, NativeEndian, U32};

const FRAME_MAGIC_NUMBER: u32 = 0x78464D45; // 'xFME' in ASCII
const DEFAULT_MAX_FRAME_LEN: usize = 16 * 1024 * 1024;

/// Errors that can occur by `FrameCodec`.
#[derive(Debug, Error)]
pub enum FrameCodecError {
    #[error("insufficient buffer capacity (required: {required}, capacity: {capacity})")]
    InsufficientBuffer { required: usize, capacity: usize },

    #[error("frame header is not aligned (alignment: {alignment})")]
    UnalignedHeader { alignment: usize },

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

/// A Framer that prefixes each payload with a magic number, its length (u32), and a CRC32 checksum.
///
/// This implementation provides a robust and high-performance framing mechanism suitable for
/// network protocols and binary data streams.
///
/// # Frame Layout
///
/// ```text
/// +----------------+------------------+-------------+-------------------+
/// |  Magic Number  |  Payload Length  |  Checksum   |     ByteSlice     |
/// |   (4 bytes)    |    (4 bytes)     |  (4 bytes)  |  (variable size)  |
/// +----------------+------------------+-------------+-------------------+
/// |                  Frame Header                   |   Frame Payload   |
/// +-------------------------------------------------+-------------------+
/// ```
///
/// - **Magic Number**: A constant value (`0x78464D45`) to identify the start of a frame.
/// - **Payload Length**: The length of the `Payload` field in bytes.
/// - **Checksum**: A CRC32 checksum calculated over the `Payload Length` and `Payload` fields.
#[derive(Debug, Clone, Copy)]
pub struct FrameCodec {
    limit: usize,
}

impl FrameCodec {
    /// Creates a new instance of `FrameCodec` with a specified maximum frame length.
    ///
    /// # Panics
    ///
    /// Panics if `limit` is smaller than the frame header size (`mem::size_of::<FrameHeader>()`),
    /// or if `limit` exceeds the maximum value representable by a `u32`.
    pub fn new(limit: usize) -> Self {
        assert!(
            limit >= mem::size_of::<FrameHeader>(),
            "max_len ({}) cannot be smaller than the frame header size ({})",
            limit,
            mem::size_of::<FrameHeader>()
        );
        assert!(
            limit <= (u32::MAX as usize),
            "max_len ({}) exceeds the maximum encodable size for a u32 length field ({})",
            limit,
            u32::MAX
        );
        Self { limit }
    }
}

impl Default for FrameCodec {
    fn default() -> Self {
        Self::new(DEFAULT_MAX_FRAME_LEN)
    }
}

impl FrameCodec {
    pub fn header_len(&self) -> usize {
        mem::size_of::<FrameHeader>()
    }

    pub fn prepare_buffer<'a>(
        &self,
        buff: &'a mut [u8],
    ) -> Result<(&'a mut [u8], &'a mut [u8]), FrameCodecError> {
        let header_len = mem::size_of::<FrameHeader>();
        let buff_len = buff.len();

        if buff_len < header_len {
            return Err(FrameCodecError::InsufficientBuffer {
                required: header_len,
                capacity: buff_len,
            });
        }

        Ok(buff.split_at_mut(header_len))
    }

    pub fn encode_frame<'a>(
        &self,
        buf: &'a mut [u8],
        payload: &'a [u8],
    ) -> Result<usize, FrameCodecError> {
        let header_len = mem::size_of::<FrameHeader>();
        let payload_len = payload.len();

        let frame_len = header_len + payload_len;
        if frame_len > self.limit {
            return Err(FrameCodecError::FrameTooLarge {
                limit: self.limit,
                actual: frame_len,
            });
        }

        let mut header = FrameHeader {
            magic: U32::new(FRAME_MAGIC_NUMBER),
            length: U32::new(payload_len as u32),
            checksum: U32::new_zeroed(),
        };

        let checksum = {
            let mut crc32 = crc32fast::Hasher::new();
            crc32.update(header.length.as_bytes());
            crc32.update(payload);
            crc32.finalize()
        };
        header.checksum.set(checksum);

        header
            .write_to_prefix(buf)
            .map_err(|_| FrameCodecError::InsufficientBuffer {
                required: header_len,
                capacity: buf.len(),
            })?;
        Ok(frame_len)
    }

    pub fn decode_frame<'a>(
        &self,
        buff: &'a [u8],
    ) -> Result<Option<(usize, &'a [u8])>, FrameCodecError> {
        let header_len = mem::size_of::<FrameHeader>();
        let buff_len = buff.len();

        /* Skip if no enough data for a full header */
        if buff_len < header_len {
            return Ok(None);
        }

        /* Parse frame header */
        let header = FrameHeader::ref_from_bytes(&buff[..header_len]).map_err(|e| match e {
            zerocopy::ConvertError::Alignment(_) => FrameCodecError::UnalignedHeader {
                alignment: mem::align_of::<FrameHeader>(),
            },
            zerocopy::ConvertError::Size(_) => FrameCodecError::InsufficientBuffer {
                required: header_len,
                capacity: buff_len,
            },
            zerocopy::ConvertError::Validity(_) => FrameCodecError::InvalidHeader,
        })?;

        /* Check frame magic number */
        let magic = header.magic.get();
        if magic != FRAME_MAGIC_NUMBER {
            return Err(FrameCodecError::MagicNumberMismatch {
                expected: FRAME_MAGIC_NUMBER,
                actual: magic,
            });
        }

        /* Check frame size */
        let frame_len = header_len + header.length.get() as usize;
        if frame_len > self.limit {
            return Err(FrameCodecError::FrameTooLarge {
                limit: self.limit,
                actual: frame_len,
            });
        }

        /* Skip if no enough data for the whole frame */
        if buff_len < frame_len {
            return Ok(None);
        }

        /* Get frame payload */
        let payload = &buff[header_len..frame_len];

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
            return Err(FrameCodecError::ChecksumMismatch {
                expected: header_checksum,
                actual: checksum,
            });
        }

        Ok(Some((frame_len, payload)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn single_frame_round_trip(framer: FrameCodec, original_payload: &[u8]) {
        // The buffer size is dynamically calculated based on the framer's header length.
        let mut encode_buffer = vec![0u8; framer.header_len() + original_payload.len() + 16];

        let payload_len = original_payload.len();

        // 1. Prepare buffer
        let (header, payload) = framer.prepare_buffer(&mut encode_buffer).unwrap();

        // 2. Write payload
        let payload = &mut payload[..payload_len];
        payload.copy_from_slice(original_payload);

        // 3. Encode frame
        let frame_len = framer.encode_frame(header, payload).unwrap();

        // 4. Decode frame
        let decode_buffer = &encode_buffer[..frame_len];
        let (consumed, decoded_payload) = framer.decode_frame(decode_buffer).unwrap().unwrap();

        // 5. Verify
        let expected_len = framer.header_len() + payload_len;
        assert_eq!(
            frame_len, expected_len,
            "Total written bytes should be header_len + payload_len"
        );
        assert_eq!(
            consumed, expected_len,
            "Consumed bytes should equal the total frame length"
        );
        assert_eq!(
            decoded_payload, original_payload,
            "Decoded payload must match the original payload"
        );
    }

    fn multi_frame_round_trip(framer: FrameCodec, payloads: &[&[u8]]) {
        let total_size: usize = payloads.iter().map(|p| framer.header_len() + p.len()).sum();
        let mut buffer = vec![0u8; total_size];

        let mut encode_pos = 0;
        for payload in payloads {
            let frame_buff = &mut buffer[encode_pos..];

            let (header, payload_buff) = framer.prepare_buffer(frame_buff).unwrap();

            let payload_len = payload.len();
            payload_buff[..payload_len].copy_from_slice(payload);

            let frame_len = framer
                .encode_frame(header, &payload_buff[..payload_len])
                .unwrap();
            encode_pos += frame_len;
        }

        let mut decode_pos = 0;
        let mut decoded_payloads = Vec::with_capacity(payloads.len());

        while decode_pos < encode_pos {
            let remaining_data = &buffer[decode_pos..];

            match framer.decode_frame(remaining_data).unwrap() {
                Some((consumed, payload)) => {
                    decoded_payloads.push(payload.to_vec());
                    decode_pos += consumed;
                }
                None => panic!("Incomplete frame at position {}", decode_pos),
            }
        }

        assert_eq!(
            payloads.len(),
            decoded_payloads.len(),
            "Decoded frame count mismatch"
        );

        for (original, decoded) in payloads.iter().zip(&decoded_payloads) {
            assert_eq!(original, decoded, "Payload content mismatch");
        }
    }

    #[test]
    fn round_trip_standard_payload() {
        single_frame_round_trip(FrameCodec::default(), b"Hello, Framer!");
    }

    #[test]
    fn round_trip_empty_payload() {
        single_frame_round_trip(FrameCodec::default(), b"");
    }

    #[test]
    fn round_trip_single_byte_payload() {
        single_frame_round_trip(FrameCodec::default(), b"\x42");
    }

    #[test]
    fn round_trip_multiple_frame() {
        let payloads: &[&[u8]] = &[
            b"",
            b"\0",
            b"\x01\x02\x03",
            b"First frame payload",
            b"The quick brown fox jumps over a lazy dog",
        ];
        multi_frame_round_trip(FrameCodec::default(), &payloads);
    }
}
