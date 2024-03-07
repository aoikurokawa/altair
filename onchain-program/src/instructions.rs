use solana_program::{entrypoint::ProgramResult, program_error::ProgramError};

pub enum Instruction {
    Create,
    Update,
}

impl Instruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        let (&bump, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Create {},
            1 => Self::Update {},
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}

pub fn create_copy_hash(program_id: &Pubkey, accounts: &[AccountInfo], bump: u8) -> ProgramResult {
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

    // let acc = &source_account;
    // let clock = Clock::from_account_info(&clock_account)?;
    // let current_slot_num = clock.slot;

    // let lamport_ref = acc.lamports.borrow();
    // let data_ref = acc.data.borrow();

    // let account_hash = account_hasher(
    //     &acc.key,
    //     **lamport_ref,
    //     &data_ref,
    //     acc.owner,
    //     acc.rent_epoch,
    // );

    // check account exist, if does not exist, create it
    let rent = Rent::get()?;
    let lamports = rent.minimum_balance(copy_account.data_len());
    invoke_signed(
        &system_instruction::create_account(
            creator_account.key,
            copy_account.key,
            lamports,
            copy_account.data_len() as u64,
            program_id,
        ),
        &[creator_account.clone(), copy_account.clone()],
        &[&[
            CopyAccount::SEED_PREFIX.as_ref(),
            source_account.key.as_ref(),
            &[bump],
        ]],
    )?;

    Ok(())
}
