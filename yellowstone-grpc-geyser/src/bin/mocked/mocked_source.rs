use crate::geyser_plugin_util::MockMessage;
use crate::mock_service;

const MOCK_BUFFER: usize = 102400;

pub fn hello() {

    let (channel_tx, mut channel_rx) = tokio::sync::mpsc::channel::<MockMessage>(MOCK_BUFFER);

    // tokio::task::spawn(yellowstone_mock_service::helloworld_traffic(channel_tx));
    tokio::task::spawn(mock_service::mainnet_traffic(
        channel_tx,
        40_000_000,
        0.5,
        // args.account_bytes_per_slot,
        // args.compressibility,
    ));


}

