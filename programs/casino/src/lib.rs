pub mod constant;
pub mod error;
pub mod processor;
pub mod state;
pub mod util;

use crate::{processor::blackjack, processor::general};
use anchor_lang::prelude::*;
use {blackjack::context::*, general::context::*};

declare_id!("6K7NpEZE9u84S5SPy69wuVACDnvahZFtozPanUBWA1PW");

#[program]
pub mod casino {
    use super::*;

    pub fn initialize_casino(ctx: Context<InitializeCasino>, chip_rate: u64) -> Result<()> {
        general::initialize_casino(ctx, chip_rate)
    }

    pub fn buy_chips(ctx: Context<BuyChips>, amount: u64) -> Result<()> {
        general::buy_chips(ctx, amount)
    }

    pub fn redeem_chips(ctx: Context<RedeemChips>, amount: u64) -> Result<()> {
        general::redeem_chips(ctx, amount)
    }
}
