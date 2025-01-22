mod mocked_source;
mod mock_service;
mod debouncer_instant;
mod geyser_plugin_util;

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc};
use yellowstone_grpc_proto::plugin::message::{CommitmentLevel, Message};

#[tokio::main]
pub async fn main() {

    let (messages_tx, messages_rx) = mpsc::unbounded_channel();
    let (broadcast_tx, broadcast_rx) = broadcast::channel(100000);

    yellowstone_grpc_geyser::grpc::GrpcService::geyser_loop(
        messages_rx,
        None,
        broadcast_tx,
    ).await;


}