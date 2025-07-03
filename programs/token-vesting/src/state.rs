use anchor_lang::prelude::*;

#[account]
pub struct VestingAccount {
    pub beneficiary: Pubkey,
    pub start_time: i64,
    pub release_time: i64,
    pub amount: u64,
    pub claimed: u64,
}
