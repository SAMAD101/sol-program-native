use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct AccountState {
    pub owner: Pubkey,
    pub balance: u64,
}

impl AccountState {
    pub const LEN: usize = 32 + 8;
}
