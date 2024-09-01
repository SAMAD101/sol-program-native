use crate::instruction::{deposit, initialize_account, withdraw};
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, msg, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
    Initialize { args: u64 },
    Deposit { amount: u64, args: u64 },
    Withdraw { args: u64 },
}

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    input: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(input).map_err(|_| {
        msg!("Failed to deserialize instruction");
        ProgramError::InvalidInstructionData
    })?;
    match instruction {
        ProgramInstruction::Initialize { args } => initialize_account(accounts, program_id),
        ProgramInstruction::Deposit { amount, args } => deposit(accounts, amount, program_id),
        ProgramInstruction::Withdraw { args } => withdraw(accounts, program_id),
    }
}
