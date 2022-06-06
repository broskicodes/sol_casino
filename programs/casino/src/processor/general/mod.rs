pub mod context;

use crate::state::*;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::{
    program::{invoke, invoke_signed},
    system_instruction,
};
use context::*;

use crate::{constant::*, error::CasinoError};

pub fn initialize_casino(ctx: Context<InitializeCasino>, chip_rate: u64) -> Result<()> {
    let wallet = &mut ctx.accounts.wallet;
    let master_vault = &mut ctx.accounts.master_vault;
    let casino = &mut ctx.accounts.casino;
    let rent = &mut ctx.accounts.rent;

    casino.key = AccountType::CasinoV1;
    casino.authority = wallet.key();
    casino.sol_per_chip_rate = chip_rate;
    casino.chips_outstanding = 0;

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

    casino.chips_outstanding += amount;

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

    casino.chips_outstanding -= amount;

    Ok(())
}
