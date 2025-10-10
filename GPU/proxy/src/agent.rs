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
use libc::gettid;
use parking_lot::Mutex;
use std::error::Error as StdError;
use std::fmt;
use std::process;
use std::sync::Once;
use tracing::debug;

use xgpu_common::ipc::{
    framer::LengthPrefixFramer,
    message::Request,
    peer::Server,
    transport::shmem::{ShmemTransport, ShmemTransportBuilder},
};

#[derive(Debug)]
pub enum AgentError {
    ServerNotInitialized,
    FmtError(fmt::Error),
}

impl fmt::Display for AgentError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgentError::ServerNotInitialized => write!(f, "Server not initialized"),
            AgentError::FmtError(e) => write!(f, "Format error: {}", e),
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
    static ref SERVER: Mutex<Option<Server<LengthPrefixFramer, ShmemTransport>>> = Mutex::new(None);
}

fn client_init(addr: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut server = SERVER.lock();
    if server.is_none() {
        let framer = LengthPrefixFramer::new(4 * 1024 * 1024);
        let transport = ShmemTransportBuilder::new()
            .buffer_size(4 * 1024 * 1024)
            .build();
        *server = Some(Server::create(framer, &transport, &addr)?);
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
    //debug!("{:#?}", resp);

    let ret_arg = *resp.ret_value();

    let ret_value = ret_arg.downcast::<T>().unwrap();
    debug!("[--->] get response ok, updating request args with OUT flag...");
    req.update_from(&resp).expect("update should succeed");

    Ok(ret_value)
}
