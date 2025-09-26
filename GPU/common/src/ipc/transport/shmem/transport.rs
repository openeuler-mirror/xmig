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

use crate::ipc::transport::{Transport, TransportError};

use super::{channel::ShmemChannel, endpoint::ShmemEndpoint};

const S2C_SUFFIX: &str = "_s2c";
const C2S_SUFFIX: &str = "_c2s";

#[derive(Debug, Clone)]
pub struct ShmemTransport {
    buffer_size: usize,
    conn_timeout: Duration,
}

impl Transport for ShmemTransport {
    type Endpoint = ShmemEndpoint;
    type Address = String;

    fn create(&self, addr: &Self::Address) -> Result<Self::Endpoint, TransportError> {
        let read_channel =
            ShmemChannel::create(format!("{}{}", addr, C2S_SUFFIX), self.buffer_size)?;
        let write_channel =
            ShmemChannel::create(format!("{}{}", addr, S2C_SUFFIX), self.buffer_size)?;

        let path = format!("shmem://{}", addr);
        let endpoint = ShmemEndpoint {
            path,
            read_channel,
            write_channel,
        };
        Ok(endpoint)
    }

    fn connect(&self, addr: &Self::Address) -> Result<Self::Endpoint, TransportError> {
        let read_path = format!("{}{}", addr, S2C_SUFFIX);
        let write_path = format!("{}{}", addr, C2S_SUFFIX);

        let read_channel = ShmemChannel::open(&read_path, self.conn_timeout)?;
        let write_channel = ShmemChannel::open(&write_path, self.conn_timeout)?;

        let path = format!("shmem://{}", addr);
        let endpoint = ShmemEndpoint {
            path,
            read_channel,
            write_channel,
        };

        Ok(endpoint)
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
