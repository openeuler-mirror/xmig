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

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
pub enum MessageError {
    #[error("Request id mismatch (expect: {expect}, actual: {actual})")]
    RequestIdMismatch { expect: u64, actual: u64 },

    #[error("Argument count mismatch (expect: {expect}, actual: {actual})")]
    ArgumentCountMismatch { expect: usize, actual: usize },

    #[error("Argument storage mismatch")]
    ArgumentStorageMismatch,

    #[error("Argument type mismatch")]
    ArgumentTypeMismatch,

    #[error("Argument type size mismatch (expect: {expect}, actual: {actual})")]
    ArgumentTypeSizeMismatch { expect: usize, actual: usize },

    #[error("Argument type alignment mismatch (expect: {expect}, actual: {actual})")]
    ArgumentTypeAlignMismatch { expect: usize, actual: usize },

    #[error("Argument type length mismatch (expect: {expect}, actual: {actual})")]
    ArgumentTypeLengthMismatch { expect: usize, actual: usize },

    #[error("Attempted to downcast non-scalar argument to scalar")]
    ArgumentIsNotScalar,

    #[error("Attempted to downcast non-slice argument to slice")]
    ArgumentIsNotSlice,

    #[error("Attempted to access unaligned data")]
    UnalignedAccess,

    #[error("Attempted to access non mutable data as mutable")]
    IllegalMutation,

    #[error("Attempted to reference inlined data")]
    IllegalBorrowOfInlined,
}
