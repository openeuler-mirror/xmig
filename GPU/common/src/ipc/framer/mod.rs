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

use std::{
    error::Error as StdError,
    ops::{Deref, DerefMut},
};

/// A type implementing this trait can be read from like a `&[u8]` and
/// must provide its frame size.
pub trait Frame: Deref<Target = [u8]> {
    fn frame_len(&self) -> usize;
}

/// A type implementing this trait can be written to like a `&mut [u8]` and
/// must provide a method to finalize the frame.
pub trait FrameBuf: Deref<Target = [u8]> + DerefMut {
    /// The error type returned upon finalization failure.
    type Error: StdError + Send + Sync + 'static;

    /// Finalizes the frame, writing the header and returning the total frame size.
    fn finalize(self, payload_len: usize) -> Result<usize, Self::Error>;
}

/// A trait for types that can encode and decode frames from byte buffers.
pub trait Framer {
    /// The error type returned by codec operations.
    type Error: StdError + Send + Sync + 'static;

    /// Prepares a new frame for writing within the given buffer.
    fn encode_frame<'a>(&self, buf: &'a mut [u8]) -> impl FrameBuf<Error = Self::Error> + 'a;

    /// Decodes a frame from the beginning of the provided buffer.
    fn decode_frame<'a>(&self, buf: &'a [u8]) -> Result<Option<impl Frame + 'a>, Self::Error>;
}

mod length_prefix_framer;
pub use length_prefix_framer::LengthPrefixFramer;
