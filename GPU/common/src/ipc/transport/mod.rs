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
    fmt::{Debug, Display},
    io::Error as IoError,
    ops::{Deref, DerefMut},
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TransportError {
    #[error("Insufficient buffer capacity (required: {required}, capacity: {capacity})")]
    InsufficientBuffer { required: usize, capacity: usize },

    #[error(
        "Consume bytes more than read buffer capacity (consume: {consume}, capacity: {capacity})"
    )]
    ReadBufferOverflow { consume: usize, capacity: usize },

    #[error(
        "Submit bytes more than write buffer capacity (submit: {submit}, capacity: {capacity})"
    )]
    WriteBufferOverflow { submit: usize, capacity: usize },

    /// An error indicating connection state is unknown.
    #[error("Invalid connection state")]
    InvalidConnectionState,

    /// An error indicating connection is not ready.
    #[error("Connection not ready")]
    ConnectionNotReady,

    /// An error indicating connection closed.
    #[error("Connection closed")]
    ConnectionClosed,

    /// An error indicating connection timeout.
    #[error("Connection timeout")]
    ConnectionTimeout,

    #[error(transparent)]
    IoError(#[from] IoError),

    #[error(transparent)]
    NixError(#[from] nix::Error),

    /// An error originating from the underlying error.
    #[error("Internal error: {0}")]
    InternalError(#[from] Box<dyn StdError + Send + Sync>),
}

pub trait ReadBuf: Deref<Target = [u8]> + DerefMut {
    fn consume(self, bytes: usize) -> Result<(), TransportError>;
}

pub trait WriteBuf: Deref<Target = [u8]> + DerefMut {
    fn submit(self, bytes: usize) -> Result<(), TransportError>;
}

pub trait Endpoint: Send + Sync {
    type ReadBuf<'a>: ReadBuf + Debug
    where
        Self: 'a;
    type WriteBuf<'a>: WriteBuf + Debug
    where
        Self: 'a;

    fn read_buf(&mut self) -> Result<Self::ReadBuf<'_>, TransportError>;
    fn write_buf(&mut self) -> Result<Self::WriteBuf<'_>, TransportError>;
}

pub trait Transport: Send + Sync {
    type Endpoint: Endpoint + Debug + Display;
    type Address: Debug + Send + Sync;

    fn create(&self, addr: &Self::Address) -> Result<Self::Endpoint, TransportError>;
    fn connect(&self, addr: &Self::Address) -> Result<Self::Endpoint, TransportError>;
}

pub mod shmem;

#[cfg(test)]
mod tests;
