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
use cudax::driver; // cuda
use cudax::nccl; // nccl
use cudax::nvml;
use cudax::runtime;
use std::os::raw::{c_int, c_uint, c_void};
use tracing::debug;
use xgpu_common::ipc::message::{Argument, ArgumentFlag};

pub struct CudaDeviceSynchronizeHandler;
impl ApiHandler for CudaDeviceSynchronizeHandler {
    fn handle_api(&self, _args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let res = unsafe { runtime::cudaDeviceSynchronize() };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaDeviceGetStreamPriorityRangeHandler;
impl ApiHandler for CudaDeviceGetStreamPriorityRangeHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let least = unsafe {
            args[0].downcast_mut::<c_int>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <least> expected: c_int".into())
            })?
        };
        let greatest = unsafe {
            args[1].downcast_mut::<c_int>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <greatest> expected: c_int".into())
            })?
        };
        let res = unsafe {
            runtime::cudaDeviceGetStreamPriorityRange(least as *mut c_int, greatest as *mut c_int)
        };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaGetLastErrorHandler;
impl ApiHandler for CudaGetLastErrorHandler {
    fn handle_api(&self, _args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let res = unsafe { runtime::cudaGetLastError() };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaPeekAtLastErrorHandler;
impl ApiHandler for CudaPeekAtLastErrorHandler {
    fn handle_api(&self, _args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        debug!("-------cudaPeekAtLastError");
        let res = unsafe { runtime::cudaPeekAtLastError() };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaGetDeviceCountHandler;
impl ApiHandler for CudaGetDeviceCountHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let count = unsafe {
            args[0].downcast_mut::<c_int>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <count> expected: c_int".into())
            })?
        };
        let res = unsafe { runtime::cudaGetDeviceCount(count as *mut i32) };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaGetDevicePropertiesV2Handler;

impl ApiHandler for CudaGetDevicePropertiesV2Handler {
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
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaDeviceGetAttributeHandler;
impl ApiHandler for CudaDeviceGetAttributeHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let value = unsafe {
            args[0].downcast_mut::<c_int>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <value> expected: c_int".into())
            })?
        };
        let attr = args[1]
            .downcast_ref::<runtime::cudaDeviceAttr>()
            .map_err(|_| {
                ServerErr::InvalidType(
                    "InvalidType, <attr> expected: runtime::cudaDeviceAttr".into(),
                )
            })?;
        let device = args[2]
            .downcast_ref::<c_int>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <device> expected: c_int".into()))?;
        let res = unsafe { runtime::cudaDeviceGetAttribute(value as *mut c_int, *attr, *device) };
        debug!("----------CudaDeviceGetAttribute");
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaSetDeviceHandler;
impl ApiHandler for CudaSetDeviceHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let device = args[0]
            .downcast_ref::<c_int>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <device> expected: c_int".into()))?;
        let res = unsafe { runtime::cudaSetDevice(*device) };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaGetDeviceHandler;
impl ApiHandler for CudaGetDeviceHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let device = unsafe {
            args[0].downcast_mut::<c_int>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <value> expected: c_int".into())
            })?
        };
        let res = unsafe { runtime::cudaGetDevice(device as *mut c_int) };
        debug!("----------CudaGetDevice res: {}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaStreamCreateWithPriorityHandler;
impl ApiHandler for CudaStreamCreateWithPriorityHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let p_stream = unsafe {
            args[0]
                .downcast_mut::<runtime::cudaStream_t>()
                .map_err(|_| {
                    ServerErr::InvalidType(
                        "InvalidType, <p_stream> expected: runtime::cudaStream_t".into(),
                    )
                })?
        };
        let flags = args[1]
            .downcast_ref::<c_uint>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <flags> expected: c_uint".into()))?;
        let priority = args[2].downcast_ref::<c_int>().map_err(|_| {
            ServerErr::InvalidType("InvalidType, <priority> expected: c_int".into())
        })?;

        let res = unsafe {
            runtime::cudaStreamCreateWithPriority(
                p_stream as *mut runtime::cudaStream_t,
                *flags,
                *priority,
            )
        };

        debug!("----------cudaStreamCreateWithPriority, res: {}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaThreadExchangeStreamCaptureModeHandler;
impl ApiHandler for CudaThreadExchangeStreamCaptureModeHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let mode = unsafe {
            args[0]
                .downcast_mut::<runtime::cudaStreamCaptureMode>()
                .map_err(|_| {
                    ServerErr::InvalidType(
                        "InvalidType, <p_stream> expected: runtime::cudaStream_t".into(),
                    )
                })?
        };
        let res = unsafe {
            runtime::cudaThreadExchangeStreamCaptureMode(
                mode as *mut runtime::cudaStreamCaptureMode,
            )
        };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaMemsetHandler;
impl ApiHandler for CudaMemsetHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let dev_ptr = unsafe {
            args[0].downcast_mut::<c_void>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <dev_ptr> expected: c_void".into())
            })?
        };
        let value = args[1]
            .downcast_ref::<c_int>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <value> expected: c_int".into()))?;
        let count = args[2]
            .downcast_ref::<usize>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <count> expected: usize".into()))?;

        let res = unsafe { runtime::cudaMemset(dev_ptr as *mut c_void, *value, *count) };

        debug!("----------cudaMemset, res: {}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CudaPointerGetAttributesHandler;
impl ApiHandler for CudaPointerGetAttributesHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let attributes = unsafe {
            args[0]
                .downcast_mut::<runtime::cudaPointerAttributes>()
                .map_err(|_| {
                    ServerErr::InvalidType(
                        "InvalidType, <attributes> expected: runtime::cudaPointerAttributes".into(),
                    )
                })?
        };
        let ptr = args[1]
            .downcast_ref::<c_void>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <ptr> expected: c_void".into()))?;
        let res = unsafe {
            runtime::cudaPointerGetAttributes(
                attributes as *mut runtime::cudaPointerAttributes,
                ptr as *const c_void,
            )
        };
        debug!("----------cudaPointerGetAttributes, res: {}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct CuDeviceGetHandler;
impl ApiHandler for CuDeviceGetHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let device = unsafe {
            args[0].downcast_mut::<driver::CUdevice>().map_err(|_| {
                ServerErr::InvalidType("InvalidType, <device> expected: driver::CUdevice".into())
            })?
        };
        let ordinal = args[1]
            .downcast_ref::<c_int>()
            .map_err(|_| ServerErr::InvalidType("InvalidType, <ordinal> expected: c_int".into()))?;
        let res = unsafe { driver::cuDeviceGet(device as *mut driver::CUdevice, *ordinal) };
        debug!("----------cuDeviceGet res: {}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}

pub struct NvmlInitV2Handler;
impl ApiHandler for NvmlInitV2Handler {
    fn handle_api(&self, _args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let res = unsafe { nvml::nvmlInit_v2() };
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
        //Ok(res)
    }
}

pub struct NcclCommDestroyHandler; // bad: type mismatch
impl ApiHandler for NcclCommDestroyHandler {
    fn handle_api(&self, args: &mut [Argument<'_>]) -> Result<Argument<'static>, ServerErr> {
        let comm = unsafe {
            args[0]
                .downcast_mut::<nccl::ncclComm>()
                .map_err(|e| debug!("{}", e))
                .expect("parse comm failed")
        };

        let res = unsafe { nccl::ncclCommDestroy(std::ptr::from_mut(comm)) };
        debug!("NcclCommDestroyHandler, res ={}", res);
        let ret_value = Argument::from_value(res, ArgumentFlag::ARG_OUT);
        Ok(ret_value)
    }
}
