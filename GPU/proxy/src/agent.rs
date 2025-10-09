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

use ctor::{ctor, dtor};
use lazy_static::lazy_static;
use parking_lot::Mutex;
use std::sync::Once;
use tracing::debug;
use cudax::runtime;
use libc::gettid;
use std::error::Error as StdError;
use std::fmt;
use std::process;
use xgpu_common::ipc::message::Request;
use xgpu_common::ipc::server::Server;
use xgpu_common::ipc::transport::shmem::{ShmemTransport, ShmemTransportBuilder};

#[derive(Debug)]
pub enum AgentError {
    ServerNotInitialized,
    FmtError(fmt::Error),
    InvokeError(String),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::ServerNotInitialized => write!(f, "Server not initialized"),
            AgentError::FmtError(e) => write!(f, "Format error: {}", e),
            AgentError::InvokeError(e) => write!(f, "Invoke failed: {}", e),
        }
    }
}

impl StdError for AgentError {}

impl From<fmt::Error> for AgentError {
    fn from(err: fmt::Error) -> Self {
        AgentError::FmtError(err)
    }
}

lazy_static! {
    static ref SERVER: Mutex<Option<Server<ShmemTransport>>> = Mutex::new(None);
}

fn client_init(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = SERVER.lock();
    if server.is_none() {
        let transport = ShmemTransportBuilder::new().build();
        *server = Some(Server::create(&transport, &addr)?);
        debug!("{:#?}", server);
    }
    Ok(())
}

fn logger_init() {
    static INIT_LOGGER: Once = Once::new();

    INIT_LOGGER.call_once(|| {
        tracing_subscriber::fmt()
            .with_max_level(tracing::Level::TRACE)
            .with_thread_ids(true)
            .with_thread_names(false)
            .with_file(false)
            .with_target(false)
            .with_writer(std::io::stdout)
            .init();
    });
}

#[ctor]
fn setup() {
    logger_init();
    client_init("1234".to_string()).expect("client_init failed");
}

#[dtor]
fn destroy() {
    SERVER.lock().take();
}

pub fn invoke_api<T: Clone + 'static + std::marker::Copy>(
    mut req: Request,
) -> Result<T, AgentError> {
    let pid = process::id();
    let tid = unsafe { gettid() };
    let tspt_addr = format!("{}_{}", pid, tid);
    debug!("transport addr: {} ", tspt_addr);
    println!("   transport addr: {} ", tspt_addr);
    debug!(
        "[<---] In invoke_api, request_id: {}, meethod_id: {}, arg_num: {}",
        req.request_id(),
        req.method_id(),
        req.args().len()
    );

    debug!("{:#?}", req);

    let mut guard = SERVER.lock();

    let server = guard.as_mut().ok_or(AgentError::ServerNotInitialized)?;

    let resp = server.invoke(&req).expect("server.invoke failed");
    debug!("{:#?}", resp);

    let ret_arg = *resp.ret_value();

    let ret_value = ret_arg.downcast::<T>().unwrap();
    debug!("[--->] get response ok");

    let out_args = resp.args();

    unsafe {
        let new_value = out_args[0]
            .downcast::<runtime::cudaDeviceProp>()
            .map_err(|_| AgentError::InvokeError("OUT type wrong----2".into()))?;

        debug!(
            "	new_value: prop major:{}, minor:{}",
            new_value.major, new_value.minor
        );
        let old_ref = req.args_mut()[0]
            .downcast_mut::<runtime::cudaDeviceProp>()
            .map_err(|e| debug!("{}", e))
            .expect("xxxxxxxxxxx");

        *old_ref = new_value;
    }

    Ok(ret_value)
}
