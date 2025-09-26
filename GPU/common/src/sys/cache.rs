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

use std::ops::{Deref, DerefMut};

/// Wrapper type to ensure cache line alignment and padding
#[repr(align(64))]
#[derive(Debug)]
pub struct CacheLineAligned<T>(pub T);

impl<T> CacheLineAligned<T> {
    pub const fn new(value: T) -> Self {
        CacheLineAligned(value)
    }
}

impl<T> Deref for CacheLineAligned<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for CacheLineAligned<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
