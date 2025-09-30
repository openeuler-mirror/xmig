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

use crate::api_handler::{ApiHandler, ServerErr};
use cudax::runtime;
use std::os::raw::c_int;
use xgpu_common::ipc::message::{Argument, ArgumentFlag};

pub struct CudaGetDevicePropertiesHandler;

impl ApiHandler for CudaGetDevicePropertiesHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let prop = unsafe {
            args[0]
                .downcast_mut::<runtime::cudaDeviceProp>()
                .map_err(|_| {
                    ServerErr::InvalidType("InvalidType, expected: cudaDeviceProp".into())
                })?
        };
        let device = unsafe {
            args[1]
                .downcast_mut::<c_int>()
                .map_err(|_| ServerErr::InvalidType("InvalidType, expected: c_int".into()))?
        };
        let res = unsafe {
            runtime::cudaGetDeviceProperties_v2(prop as *mut runtime::cudaDeviceProp, *device)
        };
        if res as i32 != 0 {
            return Err(ServerErr::ApiRunError(
                "cudaGetDeviceProperties_v2".to_string(),
                res as i32,
            ));
        }
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}
