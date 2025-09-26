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

use std::fmt::{Debug, Display};

use crate::ipc::transport::{Endpoint, TransportError};

use super::{
    buffer::{ShmemReadBuffer, ShmemWriteBuffer},
    channel::ShmemChannel,
};

/// A `Transport` implementation that uses two shared memory channels for bidirectional communication.
pub struct ShmemEndpoint {
    pub(super) path: String,
    pub(super) read_channel: ShmemChannel,
    pub(super) write_channel: ShmemChannel,
}

impl Endpoint for ShmemEndpoint {
    type ReadBuf<'a> = ShmemReadBuffer<'a>;
    type WriteBuf<'a> = ShmemWriteBuffer<'a>;

    fn read_buf(&mut self) -> Result<Self::ReadBuf<'_>, TransportError> {
        self.read_channel.read_buf()
    }

    fn write_buf(&mut self) -> Result<Self::WriteBuf<'_>, TransportError> {
        self.write_channel.write_buf()
    }
}

impl Debug for ShmemEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShmemEndpoint")
            .field("read_channel", &self.read_channel)
            .field("write_channel", &self.write_channel)
            .finish()
    }
}

impl Display for ShmemEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.path, f)
    }
}
