use agave_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoV3, SlotStatus,
};
use log::info;
use solana_sdk::account::{AccountSharedData, ReadableAccount};
use solana_sdk::commitment_config::CommitmentLevel;
use solana_sdk::transaction::SanitizedTransaction;
use std::path::Path;
use std::sync::Arc;
use solana_sdk::clock::{Epoch, Slot};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug)]
pub enum MockMessage {
    Slot(MockSlot),
    Account(MockAccount),
}

#[derive(Debug)]
pub struct MockSlot {
    pub slot: Slot,
    pub commitment_level: CommitmentLevel,
}

#[derive(Debug)]
pub struct MockAccount {
    pub slot: Slot,
    pub pubkey: Pubkey,
    pub lamports: u64,
    pub data: Vec<u8>,
    pub owner: Pubkey,
    pub executable: bool,
    pub rent_epoch: Epoch,
}

// see also GeyserPluginManager: load_plugin


pub fn accountinfo_from_shared_account_data<'a>(
    account: &'a AccountSharedData,
    txn: &'a Option<&'a SanitizedTransaction>,
    pubkey: &'a Pubkey,
    write_version: u64,
) -> ReplicaAccountInfoV3<'a> {
    ReplicaAccountInfoV3 {
        pubkey: pubkey.as_ref(),
        lamports: account.lamports(),
        owner: account.owner().as_ref(),
        executable: account.executable(),
        rent_epoch: account.rent_epoch(),
        data: account.data(),
        write_version,
        txn: *txn,
    }
}

fn setup_logger_for_plugin(new_plugin: &dyn GeyserPlugin) -> Result<(), GeyserPluginError> {
    new_plugin.setup_logger(log::logger(), log::max_level())
}

pub fn slot_status_from_commitment_level(level: CommitmentLevel) -> SlotStatus {
    match level {
        CommitmentLevel::Processed => SlotStatus::Processed,
        CommitmentLevel::Confirmed => SlotStatus::Confirmed,
        CommitmentLevel::Finalized => SlotStatus::Rooted,
    }
}
