pub mod context;

use crate::{state::*, util::*};
use anchor_lang::prelude::*;
use context::*;

pub fn initialize_blackjack_table(
    ctx: Context<InitializeBlackjackTable>,
    stakes: TableStakes,
) -> Result<()> {
    let wallet = &mut ctx.accounts.wallet;
    let table = &mut ctx.accounts.table;
    let casino = &mut ctx.accounts.casino;

    verify_authority(wallet.key, &casino.authority)?;

    table.key = AccountType::BlackjackTable;
    table.casino = casino.key();
    table.table = Table::new(stakes)?;

    Ok(())
}
