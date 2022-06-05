pub mod blackjack;
pub mod constant;
pub mod error;
pub mod state;
pub mod util;

use {
    anchor_lang::{
        prelude::*,
        solana_program::{
            program::{invoke, invoke_signed},
            system_instruction,
        },
    },
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
    std::str::FromStr,
};

use crate::{constant::*, error::CasinoError, state::*};

declare_id!("6K7NpEZE9u84S5SPy69wuVACDnvahZFtozPanUBWA1PW");

#[program]
pub mod casino {
    use super::*;

    pub fn initialize_casino(ctx: Context<InitializeCasino>, chip_rate: u64) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let master_vault = &mut ctx.accounts.master_vault;
        let casino = &mut ctx.accounts.casino;
        let rent = &mut ctx.accounts.rent;

        casino.sol_per_chip_rate = chip_rate;

        invoke(
            &system_instruction::transfer(
                wallet.key,
                master_vault.key,
                rent.minimum_balance(master_vault.data_len()),
            ),
            &[wallet.to_account_info(), master_vault.to_account_info()],
        )?;

        Ok(())
    }

    pub fn buy_chips(ctx: Context<BuyChips>, amount: u64) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let master_vault = &mut ctx.accounts.master_vault;
        let casino = &mut ctx.accounts.casino;
        let chip_mint = &mut ctx.accounts.chip_mint;
        let user_chip_account = &mut ctx.accounts.user_chip_account;

        let casino_bump = *ctx.bumps.get("casino").ok_or(CasinoError::MissingBump)?;
        let cost = amount
            .checked_mul(casino.sol_per_chip_rate)
            .ok_or(CasinoError::NumericalOverflow)?;

        invoke(
            &system_instruction::transfer(wallet.key, master_vault.key, cost),
            &[wallet.to_account_info(), master_vault.to_account_info()],
        )?;

        invoke_signed(
            &spl_token::instruction::mint_to(
                ctx.accounts.token_program.key,
                &chip_mint.key(),
                &user_chip_account.key(),
                wallet.key,
                &[],
                amount,
            )?,
            &[
                chip_mint.to_account_info(),
                user_chip_account.to_account_info(),
                wallet.to_account_info(),
            ],
            &[&[CASINO.as_bytes(), &[casino_bump]]],
        )?;

        Ok(())
    }

    pub fn redeem_chips(ctx: Context<RedeemChips>, amount: u64) -> Result<()> {
        let wallet = &mut ctx.accounts.wallet;
        let master_vault = &mut ctx.accounts.master_vault;
        let casino = &mut ctx.accounts.casino;
        let chip_mint = &mut ctx.accounts.chip_mint;
        let user_chip_account = &mut ctx.accounts.user_chip_account;

        let master_vault_bump = *ctx
            .bumps
            .get("master_vault")
            .ok_or(CasinoError::MissingBump)?;
        let payout = amount
            .checked_mul(casino.sol_per_chip_rate)
            .ok_or(CasinoError::NumericalOverflow)?;

        invoke(
            &spl_token::instruction::burn(
                ctx.accounts.token_program.key,
                &user_chip_account.key(),
                &chip_mint.key(),
                wallet.key,
                &[],
                amount,
            )?,
            &[
                chip_mint.to_account_info(),
                user_chip_account.to_account_info(),
                wallet.to_account_info(),
            ],
        )?;

        invoke_signed(
            &system_instruction::transfer(master_vault.key, wallet.key, payout),
            &[wallet.to_account_info(), master_vault.to_account_info()],
            &[&[MASTER_VAULT.as_bytes(), &[master_vault_bump]]],
        )?;

        Ok(())
    }
}

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
        space = CASINO_ACCOUNT_SPACE,
        seeds = [CASINO.as_bytes()],
        bump
    )]
    pub casino: Account<'info, Casino>,
    #[account(
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
