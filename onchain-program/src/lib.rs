use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint,
    entrypoint::ProgramResult,
    keccak::{hashv, Hash},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

mod instructions;

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    intstruction_data: &[u8],
) -> ProgramResult {
    create_copy_hash(program_id, accounts, intstruction_data[0])?;

    Ok(())
}


pub fn update_copy_hash(program_id: &Pubkey, accounts: &[AccountInfo], bump: u8) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let creator_account = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;
    let copy_account = next_account_info(accounts_iter)?;
    let system_program_account = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let (pda, _bump) = Pubkey::find_program_address(
        &[
            CopyAccount::SEED_PREFIX.as_bytes(),
            source_account.key.as_ref(),
        ],
        &program_id,
    );
    assert_eq!(copy_account.key, &pda);

    let acc = &source_account;
    let clock = Clock::from_account_info(&clock_account)?;
    let current_slot_num = clock.slot;

    let lamport_ref = acc.lamports.borrow();
    let data_ref = acc.data.borrow();

    let account_hash = account_hasher(
        &acc.key,
        **lamport_ref,
        &data_ref,
        acc.owner,
        acc.rent_epoch,
    );
    let mut ca = CopyAccount::new([0u8; 32], 0);
    ca.accumulate_hash(&account_hash.to_bytes(), current_slot_num);
    msg!(
        "slot: {:?}, triggering account hash: {:?}, accumulated hash: {:?}",
        current_slot_num,
        account_hash,
        ca.digest
    );
    Ok(())
}

pub fn account_hasher(
    pubkey: &Pubkey,
    lamports: u64,
    data: &[u8],
    owner: &Pubkey,
    rent_epoch: u64,
) -> Hash {
    hashv(&[
        pubkey.as_ref(),
        &lamports.to_le_bytes(),
        data,
        owner.as_ref(),
        &rent_epoch.to_le_bytes(),
    ])
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
