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
#![allow(clippy::missing_safety_doc)]
use cudax::nvml;
use cudax::runtime;
use std::os::raw::{c_int, c_void};
mod agent;
use agent::invoke_api;
use tracing::debug;
use xgpu_common::ipc::message::Request;
use xgpu_common::ipc::message::{Argument, ArgumentFlag};
use xgpu_common::utils::api_name::ApiFuncName;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaDeviceSynchronize() -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaDeviceSynchronize");
    let req = Request::with_args(ApiFuncName::FuncCudadevicesynchronize as u64, vec![]);
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaDeviceGetStreamPriorityRange(
    least_priority: *mut c_int,
    greatest_priority: *mut c_int,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaDeviceGetStreamPriorityRange");
    let req = Request::with_args(
        ApiFuncName::FuncCudadevicegetstreampriorityrange as u64,
        vec![
            unsafe { Argument::from_mut_ptr(least_priority, ArgumentFlag::ARG_OUT) },
            unsafe { Argument::from_mut_ptr(greatest_priority, ArgumentFlag::ARG_OUT) },
        ],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaPeekAtLastError() -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaPeekAtLastError");
    let req = Request::with_args(ApiFuncName::FuncCudapeekatlasterror as u64, vec![]);
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaGetDeviceCount(count: *mut c_int) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaGetDeviceCount");
    let req = Request::with_args(
        ApiFuncName::FuncCudagetdevicecount as u64,
        vec![unsafe { Argument::from_mut_ptr(count, ArgumentFlag::ARG_OUT) }],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

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

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaDeviceGetAttribute(
    value: *mut c_int,
    attr: runtime::cudaDeviceAttr,
    device: c_int,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaDeviceGetAttribute");
    let req = Request::with_args(
        ApiFuncName::FuncCudadevicegetattribute as u64,
        vec![
            unsafe { Argument::from_mut_ptr(value, ArgumentFlag::ARG_OUT) },
            Argument::from_ref(&attr, ArgumentFlag::ARG_IN),
            Argument::from_ref(&device, ArgumentFlag::ARG_IN),
        ],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaSetDevice(device: c_int) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaSetDevice");
    let req = Request::with_args(
        ApiFuncName::FuncCudasetdevice as u64,
        vec![Argument::from_ref(&device, ArgumentFlag::ARG_IN)],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaThreadExchangeStreamCaptureMode(
    mode: *mut runtime::cudaStreamCaptureMode,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaThreadExchangeStreamCaptureMode");
    let req = Request::with_args(
        ApiFuncName::FuncCudathreadexchangestreamcapturemode as u64,
        vec![unsafe { Argument::from_mut_ptr(mode, ArgumentFlag::ARG_OUT) }],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaMemset(
    dev_ptr: *mut c_void,
    value: c_int,
    count: usize,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaMemset");
    let req = Request::with_args(
        ApiFuncName::FuncCudamemset as u64,
        vec![
            unsafe {
                Argument::from_mut_ptr(dev_ptr, ArgumentFlag::ARG_IN | ArgumentFlag::ARG_VIRT)
            },
            Argument::from_ref(&value, ArgumentFlag::ARG_IN),
            Argument::from_ref(&count, ArgumentFlag::ARG_IN),
        ],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn cudaPointerGetAttributes(
    attributes: *mut runtime::cudaPointerAttributes,
    ptr: *const c_void,
) -> runtime::cudaError_t {
    debug!("[Hooked] api_name: cudaPointerGetAttributes");
    let req = Request::with_args(
        ApiFuncName::FuncCudapointergetattributes as u64,
        vec![
            unsafe { Argument::from_mut_ptr(attributes, ArgumentFlag::ARG_OUT) },
            unsafe { Argument::from_ptr(ptr, ArgumentFlag::ARG_IN | ArgumentFlag::ARG_VIRT) },
        ],
    );
    invoke_api::<runtime::cudaError_t>(req).expect("call invoke_api failed")
}

#[unsafe(no_mangle)] //ok
pub unsafe extern "C" fn nvmlInit_v2() -> nvml::nvmlReturn_t {
    debug!("[Hooked] api_name: nvmlInit_v2");
    //let mut args = Vec::new();
    let req = Request::with_args(ApiFuncName::FuncNvmlinitV2 as u64, vec![]);
    invoke_api::<nvml::nvmlReturn_t>(req).expect("call invoke_api failed")
}
