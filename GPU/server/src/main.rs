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

use std::env;
use tracing::debug;
use xgpu_common::ipc::client::Client;
use xgpu_common::ipc::message::{Request, Response};
use xgpu_common::ipc::transport::shmem::ShmemTransportBuilder;

mod api;
mod api_handler;
use api_handler::call_handler;

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_file(false)
        .with_writer(std::io::stdout)
        .init();

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <shmem addr>", args[0]);
        std::process::exit(1);
    }

    serve(args[1].clone());
}

fn serve(addr: String) {
    let transport = ShmemTransportBuilder::new()
        .buffer_size(4 * 1024 * 1024)
        .build();
    let mut client = Client::connect(&transport, &addr).unwrap();
    debug!("{:#?}", client);

    while let Ok(Some(mut request)) = client.receive_message::<Request>() {
        /* debug!(
            "[Server] Received request: request_id={}, method_id={}, argc={}",
            request.request_id(),
            request.method_id(),
            request.argc()
        ); */

        //debug!("{:#?}", request);

        let method_id = request.method_id();
        let ret = unsafe { call_handler(method_id, request.args_mut()) }
            .expect("[server] call_handler faied");

        let response = Response::with_request(&request, ret);

        client
            .send_message(&response)
            .expect("[server] Failed to send response");

        /* debug!(
            "[Server] Sending response: request_id={}, method_id={}",
            response.request_id(),
            response.method_id(),
        ); */
    }
}
