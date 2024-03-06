use std::{
    collections::HashMap,
    str::FromStr,
    sync::atomic::{AtomicU8, Ordering},
};

use borsh::BorshSerialize;
use config::Config;
use log::error;
use solana_geyser_plugin_interface::geyser_plugin_interface::{
    GeyserPlugin, GeyserPluginError, ReplicaAccountInfoVersions, ReplicaBlockInfoVersions,
    ReplicaTransactionInfoVersions, Result as PluginResult, SlotStatus,
};
use solana_sdk::{
    hash::{hashv, Hash},
    pubkey::Pubkey,
    slot_hashes::SlotHashes,
    vote::instruction::VoteInstruction,
};
use tokio::{
    io::AsyncWriteExt,
    net::TcpListener,
    runtime::Runtime,
    sync::{
        broadcast,
        mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender},
    },
};
use types::{
    AccountHashAccumulator, AccountInfo, BankHashProof, BlockInfo, GeyserMessage,
    SlotHashProofAccumulator, SlotInfo, TransactionInfo, TransactionSigAccumulator, Update,
    VoteAccumulator, VoteInfo,
};
use util::{
    assemble_account_delta_inclusion_proof, calculate_root_and_proofs, hash_solana_account,
};

mod config;
mod types;
mod util;

pub const SLOT_HASH_ACCOUNT: &str = "SysvarS1otHashes111111111111111111111111111";
const STARTUP_END_OF_RECEIVED: u8 = 1 << 0;
const STARTUP_PROCESSED_RECEIVED: u8 = 1 << 1;

fn handle_confirmed_slot(
    slot: u64,
    block_accumulator: &mut HashMap<u64, BlockInfo>,
    processed_slot_account_accumulator: &mut AccountHashAccumulator,
    processed_transaction_accumulator: &mut TransactionSigAccumulator,
    _processed_vote_accumulator: &mut VoteAccumulator,
    _pending_updates: &mut HashMap<Hash, Update>,
    pubkeys_for_proof: &[Pubkey],
) -> anyhow::Result<Update> {
    let Some(block) = block_accumulator.get(&slot) else {
        anyhow::bail!("block not available");
    };
    let Some(num_sigs) = processed_transaction_accumulator.get(&slot) else {
        anyhow::bail!("list of txns not available");
    };
    let Some(account_hashes_data) = processed_slot_account_accumulator.get(&slot) else {
        anyhow::bail!("account hashes not available");
    };

    let mut filtered_pubkeys: Vec<Pubkey> = pubkeys_for_proof
        .iter()
        .filter(|pubkey| account_hashes_data.contains_key(&pubkey))
        .cloned()
        .collect();

    let slothash_pubkey = Pubkey::from_str(&SLOT_HASH_ACCOUNT).unwrap();
    let slothash_account_data = account_hashes_data
        .get(&slothash_pubkey)
        .unwrap()
        .2
        .data
        .clone();
    let _slothashes: SlotHashes = bincode::deserialize(&slothash_account_data).unwrap();
    filtered_pubkeys.push(slothash_pubkey);

    if filtered_pubkeys.is_empty() {
        block_accumulator.remove(&slot);
        processed_slot_account_accumulator.remove(&slot);
        processed_transaction_accumulator.remove(&slot);
        anyhow::bail!("monitored account not modified for slot: {}", &slot);
    }

    let num_sigs = num_sigs.clone();
    let parent_bankhash = Hash::from_str(&block.parent_bankhash).unwrap();
    let blockhash = Hash::from_str(&block.blockhash).unwrap();
    let mut account_hashes: Vec<(Pubkey, Hash)> = account_hashes_data
        .iter()
        .map(|(k, (_, v, _))| (k.clone(), v.clone()))
        .collect();

    let (accounts_delta_hash, account_proofs) =
        calculate_root_and_proofs(&mut account_hashes, &pubkeys_for_proof);

    let bank_hash = hashv(&[
        parent_bankhash.as_ref(),
        accounts_delta_hash.as_ref(),
        &num_sigs.to_le_bytes(),
        blockhash.as_ref(),
    ]);

    let proofs = assemble_account_delta_inclusion_proof(
        &account_hashes_data,
        &account_proofs,
        &pubkeys_for_proof,
    )?;

    block_accumulator.remove(&slot);
    processed_slot_account_accumulator.remove(&slot);
    processed_transaction_accumulator.remove(&slot);

    Ok(Update {
        slot,
        root: bank_hash,
        proof: BankHashProof {
            proofs,
            num_sigs,
            account_delta_root: accounts_delta_hash,
            parent_bankhash,
            blockhash,
        },
    })
}

fn handle_processed_slot(
    slot: u64,
    raw_slot_account_accumulator: &mut AccountHashAccumulator,
    processed_slot_account_accumulator: &mut AccountHashAccumulator,
    raw_transaction_accumulator: &mut TransactionSigAccumulator,
    processed_transaction_accumulator: &mut TransactionSigAccumulator,
    raw_vote_accumulator: &mut VoteAccumulator,
    processed_vote_accumulator: &mut VoteAccumulator,
) -> anyhow::Result<()> {
    transfer_slot(
        slot,
        raw_slot_account_accumulator,
        processed_slot_account_accumulator,
    );
    transfer_slot(
        slot,
        raw_transaction_accumulator,
        processed_transaction_accumulator,
    );
    transfer_slot(slot, raw_vote_accumulator, processed_vote_accumulator);
    Ok(())
}

fn transfer_slot<V>(slot: u64, raw: &mut HashMap<u64, V>, processed: &mut HashMap<u64, V>) {
    if let Some(entry) = raw.remove(&slot) {
        processed.insert(slot, entry);
    }
}

async fn process_messages(
    geyser_receiver: &mut UnboundedReceiver<GeyserMessage>,
    tx: broadcast::Sender<Update>,
    pubkeys_for_proof: Vec<Pubkey>,
) {
    let mut raw_slot_account_accumulator: AccountHashAccumulator = HashMap::new();
    let mut processed_slot_account_accumulator: AccountHashAccumulator = HashMap::new();

    let mut raw_transaction_accumulator: TransactionSigAccumulator = HashMap::new();
    let mut processed_transaction_accumulator: TransactionSigAccumulator = HashMap::new();

    let mut raw_vote_accumulator: VoteAccumulator = HashMap::new();
    let mut processed_vote_accumulator: VoteAccumulator = HashMap::new();

    let _slothash_accumulator: SlotHashProofAccumulator = HashMap::new();

    let mut pending_updates: HashMap<Hash, Update> = HashMap::new();

    let mut block_accumulator: HashMap<u64, BlockInfo> = HashMap::new();

    loop {
        match geyser_receiver.recv().await {
            Some(GeyserMessage::AccountMessage(acc)) => {
                let account_hash = hash_solana_account(
                    acc.lamports,
                    acc.owner.as_ref(),
                    acc.executable,
                    acc.rent_epoch,
                    &acc.data,
                    acc.pubkey.as_ref(),
                );

                let write_version = acc.write_version;
                let slot = acc.slot;

                let slot_entry = raw_slot_account_accumulator
                    .entry(slot)
                    .or_insert_with(HashMap::new);

                let account_entry = slot_entry
                    .entry(acc.pubkey)
                    .or_insert_with(|| (0, Hash::default(), AccountInfo::default()));

                if write_version > account_entry.0 {
                    *account_entry = (write_version, Hash::from(account_hash), acc);
                }
            }
            Some(GeyserMessage::TransactionMessage(txn)) => {
                let slot_num = txn.slot;
                *raw_transaction_accumulator.entry(slot_num).or_insert(0) += txn.num_sigs;
            }
            Some(GeyserMessage::VoteMessage(vote_info)) => {
                let slot_num = vote_info.slot;
                let sig = vote_info.signature;
                raw_vote_accumulator
                    .entry(slot_num)
                    .or_insert(HashMap::new())
                    .insert(sig, vote_info);
            }
            Some(GeyserMessage::BlockMessage(block)) => {
                let slot = block.slot;
                block_accumulator.insert(
                    slot,
                    BlockInfo {
                        slot,
                        parent_bankhash: block.parent_bankhash,
                        blockhash: block.blockhash,
                        executed_transaction_count: block.executed_transaction_count,
                    },
                );
            }
            Some(GeyserMessage::SlotMessage(slot_info)) => match slot_info.status {
                SlotStatus::Processed => {
                    if let Err(e) = handle_processed_slot(
                        slot_info.slot,
                        &mut raw_slot_account_accumulator,
                        &mut processed_slot_account_accumulator,
                        &mut raw_transaction_accumulator,
                        &mut processed_transaction_accumulator,
                        &mut raw_vote_accumulator,
                        &mut processed_vote_accumulator,
                    ) {
                        error!(
                            "Error when handling processed slot {}: {:?}",
                            slot_info.slot, e
                        );
                    }
                }
                SlotStatus::Confirmed => {
                    match handle_confirmed_slot(
                        slot_info.slot,
                        &mut block_accumulator,
                        &mut processed_slot_account_accumulator,
                        &mut processed_transaction_accumulator,
                        &mut processed_vote_accumulator,
                        &mut pending_updates,
                        &pubkeys_for_proof,
                    ) {
                        Ok(update) => {
                            if let Err(e) = tx.send(update) {
                                error!(
                                    "No subscribers to receive the update {}: {:?}",
                                    slot_info.slot, e
                                );
                            }
                        }
                        Err(e) => {
                            error!("{:?}", e);
                        }
                    }
                }
                SlotStatus::Rooted => {}
            },

            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct PluginInner {
    startup_status: AtomicU8,
    geyser_sender: UnboundedSender<GeyserMessage>,
}

impl PluginInner {
    fn send_message(&self, message: GeyserMessage) {
        if let Err(e) = self.geyser_sender.send(message) {
            error!("error when sending message to geyser {:?}", e);
        }
    }
}

#[derive(Debug, Default)]
pub struct Plugin {
    inner: Option<PluginInner>,
}

impl Plugin {
    fn with_inner<F>(&self, f: F) -> PluginResult<()>
    where
        F: FnOnce(&PluginInner) -> PluginResult<()>,
    {
        let inner = self.inner.as_ref().expect("initialized");
        if inner.startup_status.load(Ordering::SeqCst)
            == STARTUP_END_OF_RECEIVED | STARTUP_PROCESSED_RECEIVED
        {
            f(inner)
        } else {
            Ok(())
        }
    }
}

impl GeyserPlugin for Plugin {
    fn name(&self) -> &'static str {
        "AccountProofGeyserPlugin"
    }

    fn on_load(&mut self, config_file: &str, _is_reload: bool) -> PluginResult<()> {
        let config = Config::load_from_file(config_file)
            .map_err(|e| GeyserPluginError::ConfigFileReadError { msg: e.to_string() })?;
        solana_logger::setup_with_default("error");

        let (geyser_sender, mut geyser_receiver) = unbounded_channel();
        let pubkeys_for_proofs: Vec<Pubkey> = config
            .account_list
            .iter()
            .map(|x| Pubkey::from_str(x).unwrap())
            .collect();

        let (tx, _rx) = broadcast::channel(32);

        let tx_process_message = tx.clone();
        std::thread::spawn(move || async move {
            process_messages(&mut geyser_receiver, tx_process_message, pubkeys_for_proofs).await;
        });

        std::thread::spawn(move || {
            let runtime = Runtime::new().unwrap();
            runtime.block_on(async {
                let listener = TcpListener::bind(&config.bind_address).await.unwrap();
                loop {
                    let (mut socket, _) = match listener.accept().await {
                        Ok(connection) => connection,
                        Err(e) => {
                            error!("Failed to accept connection: {:?}", e);
                            continue;
                        }
                    };
                    let mut rx = tx.subscribe();
                    tokio::spawn(async move {
                        match rx.recv().await {
                            Ok(update) => {
                                let mut data = Vec::new();
                                update.serialize(&mut data).unwrap();
                                let _ = socket.write_all(&data).await;
                            }
                            Err(_) => {}
                        }
                    });
                }
            });
        });

        self.inner = Some(PluginInner {
            startup_status: AtomicU8::new(0),
            geyser_sender,
        });

        Ok(())
    }

    fn on_unload(&mut self) {
        if let Some(inner) = self.inner.take() {
            drop(inner.geyser_sender);
        }
    }

    fn update_account(
        &self,
        account: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaAccountInfoVersions,
        slot: solana_sdk::slot_history::Slot,
        _is_startup: bool,
    ) -> PluginResult<()> {
        self.with_inner(|inner| {
            let account = match account {
                ReplicaAccountInfoVersions::V0_0_3(a) => a,
                _ => {
                    unreachable!("Only ReplicaAccountInfoVersions::V0_0_3 is supported")
                }
            };
            let pubkey = Pubkey::try_from(account.pubkey).unwrap();
            let owner = Pubkey::try_from(account.owner).unwrap();

            let message = GeyserMessage::AccountMessage(AccountInfo {
                pubkey,
                lamports: account.lamports,
                owner,
                executable: account.executable,
                rent_epoch: account.rent_epoch,
                data: account.data.to_vec(),
                write_version: account.write_version,
                slot,
            });
            inner.send_message(message);
            Ok(())
        })
    }

    fn notify_end_of_startup(&self) -> PluginResult<()> {
        let inner = self.inner.as_ref().expect("initialized");
        inner
            .startup_status
            .fetch_or(STARTUP_END_OF_RECEIVED, Ordering::SeqCst);
        Ok(())
    }

    fn update_slot_status(
        &self,
        slot: solana_sdk::slot_history::Slot,
        _parent: Option<u64>,
        status: SlotStatus,
    ) -> PluginResult<()> {
        let inner = self.inner.as_ref().expect("initialized");
        if inner.startup_status.load(Ordering::SeqCst) == STARTUP_END_OF_RECEIVED
            && status == SlotStatus::Processed
        {
            inner
                .startup_status
                .fetch_or(STARTUP_PROCESSED_RECEIVED, Ordering::SeqCst);
        }

        self.with_inner(|inner| {
            let message = GeyserMessage::SlotMessage(SlotInfo { slot, status });
            inner.send_message(message);
            Ok(())
        })
    }

    fn notify_transaction(
        &self,
        transaction: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaTransactionInfoVersions,
        slot: solana_sdk::slot_history::Slot,
    ) -> PluginResult<()> {
        self.with_inner(|inner| {
            let transaction = match transaction {
                ReplicaTransactionInfoVersions::V0_0_2(t) => t,
                _ => {
                    unreachable!("Only ReplicaTransactionInfoVersions::V0_0_2 is supported")
                }
            };

            if transaction.transaction.is_simple_vote_transaction() {
                match transaction.transaction.message() {
                    solana_sdk::message::SanitizedMessage::Legacy(legacy_message) => {
                        let vote_instruction: VoteInstruction =
                            bincode::deserialize(&legacy_message.message.instructions[0].data)
                                .unwrap();
                        let sig = transaction.transaction.signatures()[0];
                        match vote_instruction {
                            VoteInstruction::CompactUpdateVoteState(state_update) => {
                                let vote_message = GeyserMessage::VoteMessage(VoteInfo {
                                    slot,
                                    signature: sig,
                                    vote_for_slot: state_update.lockouts
                                        [state_update.lockouts.len() - 1]
                                        .slot(),
                                    vote_for_hash: state_update.hash,
                                    message: legacy_message.message.clone().into_owned(),
                                });
                                inner.send_message(vote_message);
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                }
            }
            let message = GeyserMessage::TransactionMessage(TransactionInfo {
                slot,
                num_sigs: transaction.transaction.signatures().len() as u64,
            });
            inner.send_message(message);
            Ok(())
        })
    }

    fn notify_entry(
        &self,
        _entry: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaEntryInfoVersions,
    ) -> PluginResult<()> {
        Ok(())
    }

    fn notify_block_metadata(
        &self,
        blockinfo: solana_geyser_plugin_interface::geyser_plugin_interface::ReplicaBlockInfoVersions,
    ) -> PluginResult<()> {
        self.with_inner(|inner| {
            let blockinfo = match blockinfo {
                ReplicaBlockInfoVersions::V0_0_2(info) => info,
                _ => {
                    unreachable!("Only ReplicaBlockInfoVersions::V0_0_1 is supported")
                }
            };
            let message = GeyserMessage::BlockMessage((blockinfo).into());
            inner.send_message(message);

            Ok(())
        })
    }

    fn account_data_notifications_enabled(&self) -> bool {
        true
    }

    fn transaction_notifications_enabled(&self) -> bool {
        true
    }

    fn entry_notifications_enabled(&self) -> bool {
        false
    }
}

#[no_mangle]
#[allow(improper_ctypes_definitions)]
/// # Safety
/// This function returns the Plugin pointer as trait GeyserPlugin.
pub unsafe extern "C" fn _create_plugin() -> *mut dyn GeyserPlugin {
    let plugin = Plugin::default();
    let plugin: Box<dyn GeyserPlugin> = Box::new(plugin);
    Box::into_raw(plugin)
}
