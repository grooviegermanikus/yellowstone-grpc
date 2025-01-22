use agave_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoV3;
use log::{debug, info, warn};
use tokio::runtime::Builder;
use tokio::sync::mpsc::UnboundedSender;
use yellowstone_grpc_proto::geyser::CommitmentLevel;
use yellowstone_grpc_proto::plugin::message;
use yellowstone_grpc_proto::plugin::message::Message;
use crate::geyser_plugin_util::MockMessage;
use crate::{debouncer_instant, mock_service};

const MOCK_BUFFER: usize = 102400;

pub fn produce_mockstream(messages_tx: UnboundedSender<message::Message>) {


    let (channel_tx, mut channel_rx) = tokio::sync::mpsc::channel::<MockMessage>(MOCK_BUFFER);

    let runtime = Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .expect("Failed to create a new runtime for geyser loop");

    runtime.spawn(mock_service::mainnet_traffic(
        channel_tx,
        40_000_000,
        0.5,
        // args.account_bytes_per_slot,
        // args.compressibility,
    ));



    let log_debouncer = debouncer_instant::Debouncer::new(std::time::Duration::from_millis(10));

    'recv_loop: loop {
        match channel_rx.blocking_recv() {
            Some(MockMessage::Account(mock_account)) => {
                // usually there are some 10-50 messages in the channel
                if log_debouncer.can_fire() {
                    info!(
                            "sending account {:?} with data_len={} ({} messags in channel)",
                            mock_account.pubkey,
                            mock_account.data.len(),
                            channel_rx.len()
                        );
                }


                let account_v3 = ReplicaAccountInfoV3 {
                    pubkey: mock_account.pubkey.as_ref(),
                    lamports: mock_account.lamports,
                    owner: mock_account.owner.as_ref(),
                    executable: mock_account.executable,
                    rent_epoch: mock_account.rent_epoch,
                    data: mock_account.data.as_ref(),
                    write_version: 999999,
                    txn: None,
                };
                let msg = message::MessageAccount::from_geyser(&account_v3, mock_account.slot, false);

                messages_tx.send(Message::Account(msg)).unwrap();
            }
            Some(MockMessage::Slot(mock_slot)) => {
                debug!(
                        "updating slot to {} with commitment {}",
                        mock_slot.slot, mock_slot.commitment_level
                    );


                let commitment_level: yellowstone_grpc_proto::plugin::message::CommitmentLevel =
                    match mock_slot.commitment_level {
                        solana_sdk::commitment_config::CommitmentLevel::Processed => {
                            message::CommitmentLevel::Processed
                        }
                        solana_sdk::commitment_config::CommitmentLevel::Confirmed => {
                            message::CommitmentLevel::Confirmed
                        }
                        solana_sdk::commitment_config::CommitmentLevel::Finalized => {
                            message::CommitmentLevel::Finalized
                        }
                    };

                let msg = message::MessageSlot {
                    slot: mock_slot.slot,
                    parent: Some(mock_slot.slot - 1),
                    status: commitment_level
                };

                messages_tx.send(message::Message::Slot(msg)).unwrap();
            }

            // plugin
            //     .update_slot_status(
            //         mock_slot.slot,
            //         None,
            //         slot_status_from_commitment_level(mock_slot.commitment_level),
            //     )
            //     .unwrap();

            // if mock_slot.commitment_level == CommitmentLevel::Processed {
            //     let block_meta = ReplicaBlockInfoV4 {
            //         parent_slot: mock_slot.slot - 1,
            //         slot: mock_slot.slot,
            //         parent_blockhash: "nohash",
            //         blockhash: "nohash",
            //         rewards: &RewardsAndNumPartitions {
            //             rewards: vec![],
            //             num_partitions: None,
            //         },
            //         block_time: None,
            //         block_height: None,
            //         executed_transaction_count: 0,
            //         entry_count: 0,
            //     };
            //     plugin
            //         .notify_block_metadata(ReplicaBlockInfoVersions::V0_0_4(&block_meta))
            //         .unwrap();
            // }
            // }
            None => {
                warn!("channel closed - shutting down");
                break 'recv_loop;
            }
        }
    }


}

