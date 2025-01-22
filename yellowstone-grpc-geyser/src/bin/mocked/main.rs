mod mocked_source;
mod mock_service;
mod debouncer_instant;
mod geyser_plugin_util;

use std::sync::Arc;
use std::thread::sleep;
use log::warn;
use tokio::runtime::Builder;
use tokio::sync::{broadcast, mpsc};
use yellowstone_grpc_proto::plugin::message::{CommitmentLevel, Message};
use crate::mocked_source::produce_mockstream;

// #[tokio::main(flavor = "multi_thread")]
pub fn main() {
    tracing_subscriber::fmt()
        // .with_env_filter(EnvFilte::from_default_env())
        .init();

    let (messages_tx, messages_rx) = mpsc::unbounded_channel();
    let (broadcast_tx, broadcast_rx) = broadcast::channel(100000);

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Failed to create a new runtime for geyser loop");

        runtime.spawn(async {
            yellowstone_grpc_geyser::grpc::GrpcService::geyser_loop(
                messages_rx,
                None,
                broadcast_tx,
            ).await;
            warn!("geyser loop exited");
    });

    produce_mockstream(messages_tx);

    sleep(std::time::Duration::from_secs(3600 * 10000));
}