use super::*;
use witnet_data_structures::chain::EpochConstants;

pub struct State {
    pub name: Option<String>,
    pub caption: Option<String>,
    pub account: u32,
    pub keychains: [types::ExtendedSK; 2],
    pub next_external_index: u32,
    pub next_internal_index: u32,
    pub available_accounts: Vec<u32>,
    pub balance: u64,
    pub transaction_next_id: u32,
    pub utxo_set: model::UtxoSet,
    pub epoch_constants: EpochConstants,
    /// Beacon of the last block received during synchronization.
    pub last_sync: CheckpointBeacon,
    /// Last block beacon confirmed by superblock (or during sync process)
    pub last_confirmed: CheckpointBeacon,
    /// List of pending balance movements, waiting to be confirmed with a superblock
    pub pending_movements: HashMap<String, Vec<model::BalanceMovement>>,
    /// List of pending address infos, waiting to be confirmed with a superblock
    pub pending_address_infos: HashMap<String, Vec<model::AddressInfo>>,
    /// List of pending blocks waiting to be confirmed
    pub pending_blocks: HashMap<String, model::Beacon>,
}
