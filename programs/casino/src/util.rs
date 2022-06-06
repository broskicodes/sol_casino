use crate::error::CasinoError;
use anchor_lang::prelude::*;

pub fn verify_authority(wallet: &Pubkey, authority: &Pubkey) -> Result<()> {
    if !wallet.eq(authority) {
        return Err(error!(CasinoError::Unauthorized));
    }

    Ok(())
}
