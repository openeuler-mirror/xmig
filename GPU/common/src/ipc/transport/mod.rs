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
    ops::{Deref, DerefMut},
};

/// A buffer trait for reading bytes from an endpoint.
pub trait ReadBuf: Debug + Deref<Target = [u8]> + DerefMut {
    /// The error type for transport operations.
    type Error: StdError + Send + Sync + 'static;

    /// Consumes the specified number of bytes from the buffer.
    fn consume(self, bytes: usize) -> Result<(), Self::Error>;
}

/// A buffer trait for writing bytes to an endpoint.
pub trait WriteBuf: Debug + Deref<Target = [u8]> + DerefMut {
    /// The error type for transport operations.
    type Error: StdError + Send + Sync + 'static;

    /// Submits the specified number of bytes to the buffer for transmission.
    fn submit(self, bytes: usize) -> Result<(), Self::Error>;
}

/// A bidirectional communication endpoint for IPC.
pub trait Endpoint: Debug + Send + Sync {
    /// The error type for transport operations.
    type Error: StdError + Send + Sync + 'static;

    /// The read buffer type for this endpoint.
    type ReadBuf<'a>: ReadBuf<Error = Self::Error> + Debug
    where
        Self: 'a;

    /// The write buffer type for this endpoint.
    type WriteBuf<'a>: WriteBuf<Error = Self::Error> + Debug
    where
        Self: 'a;

    /// Acquires a read buffer. Blocks until data is available.
    fn read(&mut self) -> Result<Self::ReadBuf<'_>, Self::Error>;

    /// Acquires a write buffer. Blocks until space is available.
    fn write(&mut self) -> Result<Self::WriteBuf<'_>, Self::Error>;
}

/// A factory for creating IPC communication endpoints.
///
/// Implementations provide transport-specific ways to create and connect
/// to endpoints using address-based identifiers. This trait serves as the
/// main entry point for establishing IPC communication channels.
pub trait Transport: Debug + Send + Sync {
    /// The address type used to identify endpoints.
    type Address: ?Sized + Send + Sync + Debug + Display;

    /// The error type for transport operations.
    type Error: StdError + Send + Sync + 'static;

    /// The endpoint type produced by this transport.
    type Endpoint: Endpoint<Error = Self::Error>;

    /// Creates a new communication endpoint bound to the given address.
    fn create(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error>;

    /// Establishes a connection to a remote endpoint at the given address.
    fn connect(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error>;
}

pub mod shmem;

#[cfg(test)]
mod tests;
