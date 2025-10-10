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

use std::time::Duration;

use crate::ipc::transport::Transport;

use super::{channel::ShmemChannel, endpoint::ShmemEndpoint, error::ShmemTransportError};

const S2C_SUFFIX: &str = "_s2c";
const C2S_SUFFIX: &str = "_c2s";

#[derive(Debug, Clone, Copy)]
pub struct ShmemTransport {
    buffer_size: usize,
    conn_timeout: Duration,
}

impl Transport for ShmemTransport {
    type Error = ShmemTransportError;
    type Endpoint = ShmemEndpoint;
    type Address = str;

    fn create(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error> {
        let tx = ShmemChannel::create(format!("{}{}", addr, S2C_SUFFIX), self.buffer_size)?;
        let rx = ShmemChannel::create(format!("{}{}", addr, C2S_SUFFIX), self.buffer_size)?;

        Ok(ShmemEndpoint::new(tx, rx))
    }

    fn connect(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error> {
        let tx = ShmemChannel::open(format!("{}{}", addr, C2S_SUFFIX), self.conn_timeout)?;
        let rx = ShmemChannel::open(format!("{}{}", addr, S2C_SUFFIX), self.conn_timeout)?;

        Ok(ShmemEndpoint::new(tx, rx))
    }
}

#[derive(Debug, Clone)]
pub struct ShmemTransportBuilder {
    buffer_size: usize,
    conn_timeout: Duration,
}

impl ShmemTransportBuilder {
    pub fn new() -> Self {
        const DEFAULT_BUFF_SIZE: usize = 4096;
        const DEFAULT_CONN_TIMEOUT: Duration = Duration::from_millis(100);

        Self {
            buffer_size: DEFAULT_BUFF_SIZE,
            conn_timeout: DEFAULT_CONN_TIMEOUT,
        }
    }

    #[inline]
    pub fn buffer_size(mut self, value: usize) -> Self {
        self.buffer_size = value;
        self
    }

    #[inline]
    pub fn connect_timeout(mut self, value: Duration) -> Self {
        self.conn_timeout = value;
        self
    }

    #[inline]
    pub fn build(self) -> ShmemTransport {
        ShmemTransport {
            buffer_size: self.buffer_size,
            conn_timeout: self.conn_timeout,
        }
    }
}

impl Default for ShmemTransportBuilder {
    fn default() -> Self {
        Self::new()
    }
}
