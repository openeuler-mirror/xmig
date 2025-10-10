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

//! A shared-memory (SHMEM) transport layer implementation.
//!
//! This module provides a high-performance, inter-process communication mechanism
//! built on shared memory ring buffers. It uses one channel for each direction of
//! communication. Synchronization is managed by `parking_lot` primitives placed
//! within the shared memory segment itself.

mod error;
pub use error::*;

mod memory;

mod buffer;
mod channel;

mod endpoint;
pub use endpoint::*;

mod transport;
pub use transport::*;
