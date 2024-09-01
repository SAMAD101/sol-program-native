use crate::state::AccountState;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction, system_program,
};

pub fn initialize_account(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;
    let owner = next_account_info(account_iter)?;
    let system_program_info = next_account_info(account_iter)?;

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if system_program_info.key != &system_program::ID {
        return Err(ProgramError::IncorrectProgramId);
    }

    let rent = Rent::default();

    if account.lamports() == 0 {
        msg!("Creating new account");
        let space = AccountState::LEN;
        let lamports = rent.minimum_balance(space);
        invoke(
            &system_instruction::create_account(
                owner.key,
                account.key,
                lamports,
                space as u64,
                program_id,
            ),
            &[owner.clone(), account.clone(), system_program_info.clone()],
        )?;
    } else {
        msg!("Account already exists, checking ownership");
        if account.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }
    }

    if !rent.is_exempt(account.lamports(), account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    let account_state = AccountState {
        owner: *owner.key,
        balance: 0,
    };

    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    msg!("Account initialized successfully");

    Ok(())
}

pub fn deposit(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;
    let owner = next_account_info(account_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_state = AccountState::try_from_slice(&account.data.borrow())?;

    if account_state.owner != *owner.key {
        return Err(ProgramError::InvalidAccountData);
    }

    account_state.balance += amount;
    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    **account.try_borrow_mut_lamports()? += amount;
    **owner.try_borrow_mut_lamports()? -= amount;

    Ok(())
}

pub fn withdraw(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
    let account_iter = &mut accounts.iter();
    let account = next_account_info(account_iter)?;
    let owner = next_account_info(account_iter)?;

    if account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }

    if !owner.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut account_state = AccountState::try_from_slice(&account.data.borrow())?;

    if account_state.owner != *owner.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let withdraw_amount = account_state.balance / 10; // 10% of the balance
    account_state.balance -= withdraw_amount;
    account_state.serialize(&mut &mut account.data.borrow_mut()[..])?;

    **account.try_borrow_mut_lamports()? -= withdraw_amount;
    **owner.try_borrow_mut_lamports()? += withdraw_amount;

    Ok(())
}
