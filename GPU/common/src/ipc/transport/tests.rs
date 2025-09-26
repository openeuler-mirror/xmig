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

use super::*;

mod helper {
    use std::sync::{Arc, Once};

    use tracing::debug;

    use super::*;

    pub fn init_test_logger() {
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

    pub fn create_connection<T: Transport + 'static>(
        transport: T,
        addr: T::Address,
    ) -> Result<(T::Endpoint, T::Endpoint), TransportError> {
        let transport = Arc::new(transport);

        let server = transport.create(&addr)?;
        debug!("Created '{}'", server);
        debug!("Server: {:#?}", server);

        let client = transport.connect(&addr)?;
        debug!("Connected to '{}'", client);
        debug!("Client: {:#?}", client);

        Ok((server, client))
    }

    pub fn send_message<E: Endpoint>(
        endpoint: &mut E,
        message: &[u8],
    ) -> Result<(), TransportError> {
        let msg_len = message.len();

        debug!("Sending message '{}'", String::from_utf8_lossy(message));

        // Write message to buffer
        let mut write_buf = endpoint.write_buf()?;
        write_buf[..message.len()].copy_from_slice(message);

        // Submit changes
        write_buf.submit(message.len())?;
        debug!("Sent {} bytes", msg_len);

        Ok(())
    }

    pub fn receive_message<E: Endpoint>(
        endpoint: &mut E,
        expected: &[u8],
    ) -> Result<(), TransportError> {
        let msg_len = expected.len();

        debug!("Receiving message...");

        // Read message from buffer
        let read_buf = endpoint.read_buf()?;
        let message = &read_buf[..expected.len()];

        // Validate message
        debug!("Received message '{}'", String::from_utf8_lossy(message));
        assert_eq!(message, expected, "Received message mismatch");

        // Consume read buffer
        read_buf.consume(expected.len())?;
        debug!("Received {} bytes", msg_len);

        Ok(())
    }

    pub fn send_raw_bytes<E: Endpoint>(
        endpoint: &mut E,
        message: &[u8],
    ) -> Result<(), TransportError> {
        let total_len = message.len();
        let mut bytes_sent = 0;

        while bytes_sent < total_len {
            let mut write_buf = endpoint.write_buf()?;

            // Determine the size of the next chunk to send
            let chunk_size = std::cmp::min(
                write_buf.len(),        // Buffer capacity
                total_len - bytes_sent, // Remaining bytes
            );

            let chunk = &message[bytes_sent..bytes_sent + chunk_size];
            write_buf[..chunk.len()].copy_from_slice(chunk);

            write_buf.submit(chunk.len())?;
            bytes_sent += chunk.len();

            debug!("Sent: {}/{}, chunk: {}", bytes_sent, total_len, chunk_size);
        }

        Ok(())
    }

    pub fn recv_raw_bytes<E: Endpoint>(
        endpoint: &mut E,
        expected: &[u8],
    ) -> Result<Vec<u8>, TransportError> {
        let expected_len = expected.len();
        let mut received_data = Vec::with_capacity(expected_len);

        while received_data.len() < expected_len {
            let read_buf = endpoint.read_buf()?;

            let received_len = read_buf.len();
            if received_len == 0 {
                return Err(TransportError::ConnectionClosed);
            }
            received_data.extend_from_slice(&read_buf);

            debug!(
                "Recv: {}/{}, chunk: {}",
                received_data.len(),
                expected_len,
                received_len
            );
            read_buf.consume(received_len)?;
        }

        Ok(received_data)
    }
}

mod test_suits {
    use std::{sync::Arc, thread, time::Duration};

    use tracing::debug;

    use super::*;

    pub fn bidirectional_communication<T: Transport + 'static>(
        transport: T,
        addr: T::Address,
    ) -> Result<(), TransportError> {
        const PING: &[u8] = b"Ping";
        const PONG: &[u8] = b"Pong";

        // Initialize logger
        helper::init_test_logger();

        // Create server & client
        let (mut server, mut client) = helper::create_connection(transport, addr)?;

        // Client send PING
        helper::send_message(&mut client, PING)?;

        // Server receive PING
        helper::receive_message(&mut server, PING)?;

        // Server send PONG
        helper::send_message(&mut server, PONG)?;

        // Client receive PING
        helper::receive_message(&mut client, PONG)?;

        Ok(())
    }

    pub fn transfer_raw_bytes<T: Transport + 'static>(
        transport: T,
        addr: T::Address,
        data_size: usize,
    ) -> Result<(), TransportError> {
        // Initialize logger
        helper::init_test_logger();

        // Generate predictable, non-UTF8 raw byte data.
        // The pattern is a repeating sequence of 0-255.
        debug!("Generating test data...");
        let test_data = Arc::new((0..data_size).map(|i| (i % 256) as u8).collect::<Vec<_>>());

        // Create server & client
        let (mut server, mut client) = helper::create_connection(transport, addr)?;

        // Clone the data for the receiver thread
        let expected_data = test_data.clone();

        // Start sender thread
        let client_thread = {
            thread::Builder::new()
                .name("sender".to_string())
                .spawn(move || {
                    let total_bytes = test_data.len();

                    debug!("Sending {} bytes...", total_bytes);
                    helper::send_raw_bytes(&mut client, &test_data).expect("Send raw bytes failed");

                    debug!("Sent {} bytes", total_bytes);
                    thread::sleep(Duration::from_millis(200));
                })
                .expect("failed to start sender thread")
        };

        // Start receiver thread
        let server_thread = {
            thread::Builder::new()
                .name("receiver".to_string())
                .spawn(move || {
                    let total_bytes = expected_data.len();

                    debug!("Receiving {} bytes...", total_bytes);
                    let received_data = helper::recv_raw_bytes(&mut server, &expected_data)
                        .expect("Receive raw bytes failed");

                    debug!("Received {} bytes", total_bytes);
                    assert_eq!(
                        received_data,
                        expected_data.as_slice(),
                        "Received bytes mismatch"
                    );
                })
                .expect("failed to start receiver thread")
        };

        // Wait for all threads to finish
        client_thread.join().expect("sender thread panicked");
        server_thread.join().expect("receiver thread panicked");

        Ok(())
    }
}

mod shmem {
    use std::{
        ffi::OsStr,
        process,
        sync::{
            OnceLock,
            atomic::{AtomicUsize, Ordering},
        },
        time::{Duration, SystemTime, UNIX_EPOCH},
    };

    use crate::ipc::transport::shmem::ShmemTransportBuilder;

    use super::*;

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
    fn test_bidirectional_communication() -> Result<(), TransportError> {
        let transport = ShmemTransportBuilder::default().build();
        let addr = unique_shmem_addr();

        test_suits::bidirectional_communication(transport, addr)
    }

    #[test]
    fn test_raw_bytes_transfer() -> Result<(), TransportError> {
        const DATA_SIZE: usize = 16 * 1024 * 1024; // 16M
        const BUFFER_SIZE: usize = 16 * 1024; // 16K
        const TEST_TIMEOUT: Duration = Duration::from_millis(200);

        let transport = ShmemTransportBuilder::new()
            .buffer_size(BUFFER_SIZE)
            .connect_timeout(TEST_TIMEOUT)
            .build();
        let addr = unique_shmem_addr();

        test_suits::transfer_raw_bytes(transport, addr, DATA_SIZE)
    }
}
