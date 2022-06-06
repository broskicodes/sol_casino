use crate::{constant::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(stakes: TableStakes)]
pub struct InitializeBlackjackTable<'info> {
    #[account(
        init,
        payer = wallet,
        space = BLACKJACK_TABLE_ACCOUNT_SIZE,
        seeds = [
            TABLE.as_bytes(),
            casino.key().as_ref(),
            &[stakes as u8],
        ],
        bump,
    )]
    pub table: Account<'info, BlackjackTable>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        mut,
        seeds = [CASINO.as_bytes()],
        bump,
    )]
    pub casino: Account<'info, Casino>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
