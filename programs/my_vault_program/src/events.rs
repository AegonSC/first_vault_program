use anchor_lang::prelude::*;

#[event]
pub struct DepositEvent {
    pub initializer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct WithdrawEvent {
    pub initializer: Pubkey,
    pub amount: u64,
}

#[event]
pub struct CloseVaultEvent {
    pub initializer: Pubkey,
    pub amount: u64,
}