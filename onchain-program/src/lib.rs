use borsh::{BorshDeserialize, BorshSerialize};
use instructions::{create_copy_hash, update_copy_hash, Instruction};
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, keccak::hashv, pubkey::Pubkey,
};

pub mod instructions;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    intstruction_data: &[u8],
) -> ProgramResult {
    let instruction = Instruction::unpack(intstruction_data)?;

    match instruction {
        Instruction::Create { bump } => {
            create_copy_hash(program_id, accounts, bump)?;
        }
        Instruction::Update { bump } => update_copy_hash(program_id, accounts, bump)?,
    }

    Ok(())
}

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CopyAccount {
    pub digest: [u8; 32],
    slot: u64,
}

impl CopyAccount {
    pub const ACCOUNT_SPACE: usize = (4 + 1 * 32) + 8;

    pub const SEED_PREFIX: &'static str = "copy_hash";

    pub fn new(digest: [u8; 32], slot: u64) -> Self {
        Self { digest, slot }
    }

    pub fn accumulate_hash(&mut self, account_hash: &[u8; 32], slot_num: u64) {
        if slot_num > self.slot {
            self.digest = *account_hash;
            self.slot = slot_num;
        } else {
            self.digest = digest_accumulator(&self.digest, account_hash);
        }
    }
}

fn digest_accumulator(current_hash: &[u8; 32], digest: &[u8; 32]) -> [u8; 32] {
    let combined = [current_hash.as_ref(), digest.as_ref()];
    hashv(&combined).0
}
