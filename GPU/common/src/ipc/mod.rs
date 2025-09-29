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

pub mod codec;
pub mod transport;

pub mod bytewise;
pub mod message;

pub mod client;
pub mod error;
pub mod server;

#[cfg(test)]
mod tests {
    use std::{
        ffi::OsStr,
        process,
        sync::{
            Once, OnceLock,
            atomic::{AtomicUsize, Ordering},
        },
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use tracing::debug;

    use crate::ipc::{
        client::Client,
        message::{Argument, ArgumentFlag, Request, Response},
        server::Server,
        transport::shmem::ShmemTransportBuilder,
    };

    fn init_test_logger() {
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

    fn unique_shmem_addr() -> String {
        static SEQ: AtomicUsize = AtomicUsize::new(0);
        static EXEC_NAME: OnceLock<String> = OnceLock::new();

        let exec_name = EXEC_NAME.get_or_init(|| {
            std::env::current_exe()
                .ok()
                .and_then(|p| p.file_stem().and_then(OsStr::to_str).map(str::to_owned))
                .unwrap_or_else(|| "unknown".into())
        });
        let curr_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos())
            .expect("Failed to get current time");

        format!(
            "/{}_{}_{}_{}",
            exec_name,
            process::id(),
            curr_time,
            SEQ.fetch_add(1, Ordering::Relaxed)
        )
    }

    #[test]
    fn test_invoke() {
        mod method_id {
            pub const ADD_U64: u64 = 0xCAFE;
            pub const SHUTDOWN: u64 = 0xFFFF;
        }

        self::init_test_logger();

        let transport = ShmemTransportBuilder::new().build();
        let addr = unique_shmem_addr();

        let mut server = Server::create(&transport, &addr).unwrap();
        debug!("{:?}", server);

        let mut client = Client::connect(&transport, &addr).unwrap();
        debug!("{:?}", client);

        let server_thread = std::thread::spawn(move || {
            debug!("[Server] Thread started");

            while let Ok(Some(request)) = server.receive_message::<Request>() {
                debug!(
                    "[Server] Received request: request_id={}, method_id={}, argc={}",
                    request.request_id(),
                    request.method_id(),
                    request.argc()
                );

                match request.method_id() {
                    method_id::ADD_U64 => {
                        let lhs = request.args()[0]
                            .downcast::<u64>()
                            .expect("Invalid argument type");
                        let rhs = request.args()[1]
                            .downcast::<u64>()
                            .expect("Invalid argument type");

                        let result = lhs + rhs;
                        let response = Response::with_request(
                            &request,
                            Argument::from_ref(&result, ArgumentFlag::default()),
                        );
                        debug!(
                            "[Server] Sending response: request_id={}, method_id={}",
                            response.request_id(),
                            response.method_id(),
                        );
                        server
                            .send_message(&response)
                            .expect("Failed to send response");
                    }
                    method_id::SHUTDOWN => {
                        let response = Response::with_request(&request, Argument::empty());
                        debug!(
                            "[Server] Sending response: request_id={}, method_id={}",
                            response.request_id(),
                            response.method_id(),
                        );
                        server
                            .send_message(&response)
                            .expect("Failed to send response");

                        debug!("[Server] Received shutdown request, shutting down...");
                        break;
                    }
                    _ => {
                        debug!(
                            "[Server] Request {}: Unknown method {}",
                            request.request_id(),
                            request.method_id(),
                        );
                        break;
                    }
                }
            }

            thread::sleep(Duration::from_millis(200));
            debug!("[Server] Thread finished");
        });

        {
            let mut value = 0u64;
            for i in 1..=100u64 {
                debug!("[Client] Invoking 'server::add_u64({}, {})'...", value, i);
                let request = Request::with_args(
                    method_id::ADD_U64,
                    vec![
                        Argument::from_ref(&value, ArgumentFlag::ARG_IN),
                        Argument::from_ref(&i, ArgumentFlag::ARG_IN),
                    ],
                );
                let response = client.invoke(&request).expect("Invoke failed");

                let ret_val = response
                    .ret_value()
                    .downcast::<u64>()
                    .expect("Invalid argument type");
                debug!(
                    "[Client] Result 'server::add_u64({}, {})' is {}",
                    value, i, ret_val
                );

                value = ret_val;
            }
        }

        {
            let request = Request::empty(method_id::SHUTDOWN);
            debug!("[Client] Invoking 'server::shutdown()'...");
            let response = client.invoke(&request).expect("Invoke failed");
            let ret_val = response
                .ret_value()
                .downcast::<()>()
                .expect("Invalid argument type");
            debug!("[Client] Result 'server::shutdown()' is {:?}", ret_val);
        }

        let _ = server_thread.join();
    }
}
