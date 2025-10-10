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

pub trait ReadBuf: Debug + Deref<Target = [u8]> + DerefMut {
    type Error: StdError + Send + Sync + 'static;

    /// Consumes the specified number of bytes from the buffer.
    fn consume(self, bytes: usize) -> Result<(), Self::Error>;
}

pub trait WriteBuf: Debug + Deref<Target = [u8]> + DerefMut {
    type Error: StdError + Send + Sync + 'static;

    /// Submits the specified number of bytes to the buffer.
    fn submit(self, bytes: usize) -> Result<(), Self::Error>;
}

pub trait Endpoint: Debug + Send + Sync {
    type Error: StdError + Send + Sync + 'static;

    type ReadBuf<'a>: ReadBuf<Error = Self::Error> + Debug
    where
        Self: 'a;
    type WriteBuf<'a>: WriteBuf<Error = Self::Error> + Debug
    where
        Self: 'a;

    /// Acquires a read buffer. Blocks until data is available.
    fn read_buf(&mut self) -> Result<Self::ReadBuf<'_>, Self::Error>;

    /// Acquires a write buffer. Blocks until space is available.
    fn write_buf(&mut self) -> Result<Self::WriteBuf<'_>, Self::Error>;
}

pub trait Transport: Debug + Send + Sync {
    type Address: ?Sized + Send + Sync + Debug + Display;
    type Error: StdError + Send + Sync + 'static;

    type Endpoint: Endpoint<Error = Self::Error>;

    /// Creates a new communication endpoint bound to the given address.
    fn create(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error>;

    /// Establishes a connection to a remote endpoint at the given address.
    fn connect(&self, addr: &Self::Address) -> Result<Self::Endpoint, Self::Error>;
}

pub mod shmem;

#[cfg(test)]
mod tests;
