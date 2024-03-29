use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    clock::Clock,
    entrypoint::ProgramResult,
    hash::{hashv, Hash},
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::CopyAccount;

pub enum Instruction {
    Create { bump: u8 },
    Update { bump: u8 },
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, _) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let (&bump, _) = input.split_first().ok_or(ProgramError::InvalidArgument)?;

        Ok(match variant {
            0 => Self::Create { bump },
            1 => Self::Update { bump },
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

pub fn create_copy_hash(program_id: &Pubkey, accounts: &[AccountInfo], _bump: u8) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let creator_account = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;
    let copy_account = next_account_info(accounts_iter)?;
    let _system_program = next_account_info(accounts_iter)?;
    let clock_account = next_account_info(accounts_iter)?;

    let (pda, bump) = Pubkey::find_program_address(
        &[
            CopyAccount::SEED_PREFIX.as_bytes(),
            source_account.key.as_ref(),
        ],
        &program_id,
    );
    assert_eq!(copy_account.key, &pda);

    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(CopyAccount::ACCOUNT_SPACE);

    msg!("Creating Copy Account at {:?}", pda);
    invoke_signed(
        &system_instruction::create_account(
            creator_account.key,
            copy_account.key,
            lamports,
            CopyAccount::ACCOUNT_SPACE as u64,
            program_id,
        ),
        &[creator_account.clone(), copy_account.clone()],
        &[&[
            CopyAccount::SEED_PREFIX.as_ref(),
            source_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    let mut account_data =
        try_from_slice_unchecked::<CopyAccount>(&copy_account.data.borrow()).unwrap();

    account_data.digest = [0u8; 32];

    let clock = Clock::from_account_info(&clock_account)?;
    let current_slot_num = clock.slot;
    account_data.slot = current_slot_num;

    account_data.serialize(&mut &mut copy_account.data.borrow_mut()[..])?;

    Ok(())
}

pub fn update_copy_hash(program_id: &Pubkey, accounts: &[AccountInfo], _bump: u8) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let _creator_account = next_account_info(accounts_iter)?;
    let source_account = next_account_info(accounts_iter)?;
    let copy_account = next_account_info(accounts_iter)?;
    let _system_program_account = next_account_info(accounts_iter)?;
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

    let mut ca = try_from_slice_unchecked::<CopyAccount>(&copy_account.data.borrow()).unwrap();
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
