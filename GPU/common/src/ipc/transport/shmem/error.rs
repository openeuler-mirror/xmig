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

use thiserror::Error;

/// Shared memory transport error types.
#[derive(Error, Debug)]
pub enum ShmemTransportError {
    #[error("Failed to create shared memory '{name}', {source}")]
    CreationError {
        name: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to open shared memory '{name}', {source}")]
    OpenError {
        name: String,
        #[source]
        source: std::io::Error,
    },

    /// Indicates connection state is unknown or invalid.
    #[error("Invalid connection state")]
    InvalidConnectionState,

    /// Indicates connection is not ready for operation.
    #[error("Connection not ready")]
    ConnectionNotReady,

    /// Indicates connection was closed.
    #[error("Connection closed")]
    ConnectionClosed,

    /// Indicates connection operation timed out.
    #[error("Connection timeout")]
    ConnectionTimeout,

    /// Insufficient buffer capacity for operation.
    #[error("Insufficient buffer capacity (required: {required}, capacity: {capacity})")]
    InsufficientBuffer { required: usize, capacity: usize },

    /// Attempted to read beyond buffer capacity.
    #[error("Read buffer overflow (attempted: {attempted}, capacity: {capacity})")]
    ReadOverflow { attempted: usize, capacity: usize },

    /// Attempted to write beyond buffer capacity.
    #[error("Write buffer overflow (attempted: {attempted}, capacity: {capacity})")]
    WriteOverflow { attempted: usize, capacity: usize },
}
