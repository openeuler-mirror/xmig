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

use cudax::runtime;
use std::os::raw::c_int;
mod agent;
use agent::invoke_api;
use tracing::debug;
use xgpu_common::ipc::message::Request;
use xgpu_common::ipc::message::{Argument, ArgumentFlag};
use xgpu_common::utils::api_name::ApiFuncName;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaGetDeviceProperties_v2(
    prop: *mut runtime::cudaDeviceProp,
    mut device: c_int,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaGetDeviceProperties_v2");
    let req = Request::with_args(
        ApiFuncName::FuncCudagetdevicepropertiesV2 as u64,
        vec![
            unsafe { Argument::from_mut_ptr(prop, ArgumentFlag::ARG_OUT) },
            Argument::from_mut(&mut device, ArgumentFlag::ARG_IN),
        ],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}
