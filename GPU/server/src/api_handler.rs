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

use crate::api::*;
use indexmap::{IndexMap, indexmap};
use lazy_static::lazy_static;
use std::fmt;
use xgpu_common::ipc::message::Argument;
use xgpu_common::utils::api_name::ApiFuncName;

#[derive(Debug)]
pub enum ServerErr {
    ApiRunError(String, i32),
    InvalidType(String),
}

impl fmt::Display for ServerErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServerErr::ApiRunError(msg, err_code) => {
                write!(f, "API run failed, api: {}, error code: {}", msg, err_code)
            }
            ServerErr::InvalidType(msg) => write!(f, "Invalid type: {}", msg),
        }
    }
}

pub trait ApiHandler: Send + Sync {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr>;
}

lazy_static! {
    pub static ref FUNC_HANDLER_MAP: IndexMap<u64, Box<dyn ApiHandler>> = indexmap! {
        (ApiFuncName::FuncCudagetdevicepropertiesV2 as u64) => Box::new(CudaGetDevicePropertiesHandler) as Box<dyn ApiHandler>,
    };
}

pub unsafe fn call_handler(
    key: u64,
    args: &mut [Argument<'_>],
) -> Result<Argument<'static>, ServerErr> {
    match FUNC_HANDLER_MAP.get(&key) {
        Some(handler) => handler.handle_api(args),
        None => Err(ServerErr::ApiRunError(
            "call_handler.get failed".to_string(),
            -1,
        )),
    }
}
