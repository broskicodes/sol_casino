use anchor_lang::prelude::*;

// Accounts
pub const CASINO_ACCOUNT_SPACE: usize = 8 + 8;
#[account]
pub struct Casino {
    pub sol_per_chip_rate: u64,
}
