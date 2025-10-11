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

use std::fmt::Debug;

use crate::ipc::transport::Endpoint;

use super::{
    buffer::{ShmemReadBuffer, ShmemWriteBuffer},
    channel::ShmemChannel,
    error::ShmemTransportError,
};

/// A `Transport` implementation that uses two shared memory channels for bidirectional communication.
pub struct ShmemEndpoint {
    tx: ShmemChannel,
    rx: ShmemChannel,
}

impl ShmemEndpoint {
    #[inline]
    pub(crate) fn new(tx: ShmemChannel, rx: ShmemChannel) -> Self {
        Self { tx, rx }
    }
}

impl Endpoint for ShmemEndpoint {
    type Error = ShmemTransportError;
    type ReadBuf<'a> = ShmemReadBuffer<'a>;
    type WriteBuf<'a> = ShmemWriteBuffer<'a>;

    fn read(&mut self) -> Result<Self::ReadBuf<'_>, Self::Error> {
        self.rx.read_buf()
    }

    fn write(&mut self) -> Result<Self::WriteBuf<'_>, Self::Error> {
        self.tx.write_buf()
    }
}

impl Debug for ShmemEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ShmemEndpoint")
            .field("tx", &self.tx)
            .field("rx", &self.rx)
            .finish()
    }
}
