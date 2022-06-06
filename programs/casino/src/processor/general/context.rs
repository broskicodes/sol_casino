use crate::{constant::*, error::*, state::*};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};
use std::str::FromStr;

#[derive(Accounts)]
pub struct InitializeCasino<'info> {
    #[account(
        mut,
        address = Pubkey::from_str(ADMIN_ADDRESS).unwrap() @ CasinoError::Unauthorized
    )]
    pub wallet: Signer<'info>,
    #[account(
        init,
        payer = wallet,
        space = CASINO_ACCOUNT_SIZE,
        seeds = [CASINO.as_bytes()],
        bump
    )]
    pub casino: Account<'info, Casino>,
    #[account(
        mut,
        seeds = [MASTER_VAULT.as_bytes()],
        bump
    )]
    pub master_vault: SystemAccount<'info>,
    #[account(
        init,
        payer = wallet,
        seeds = [
            CHIP.as_bytes(),
            casino.key().as_ref(),
        ],
        bump,
        mint::authority = casino,
        mint::decimals = 0,
    )]
    pub chip_mint: Account<'info, Mint>,
    // TODO: Init a usdc token account for master vault
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct BuyChips<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        init_if_needed,
        payer = wallet,
        associated_token::mint = chip_mint,
        associated_token::authority = wallet,
    )]
    pub user_chip_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [CASINO.as_bytes()],
        bump,
    )]
    pub casino: Account<'info, Casino>,
    #[account(
        mut,
        seeds = [MASTER_VAULT.as_bytes()],
        bump
    )]
    pub master_vault: SystemAccount<'info>,
    #[account(
        seeds = [
            CHIP.as_bytes(),
            casino.key().as_ref(),
        ],
        bump
    )]
    pub chip_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct RedeemChips<'info> {
    #[account(mut)]
    pub wallet: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = chip_mint,
        associated_token::authority = wallet,
    )]
    pub user_chip_account: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [CASINO.as_bytes()],
        bump,
    )]
    pub casino: Account<'info, Casino>,
    #[account(
        mut,
        seeds = [MASTER_VAULT.as_bytes()],
        bump
    )]
    pub master_vault: SystemAccount<'info>,
    #[account(
        seeds = [
            CHIP.as_bytes(),
            casino.key().as_ref(),
        ],
        bump
    )]
    pub chip_mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}
