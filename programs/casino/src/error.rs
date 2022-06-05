use anchor_lang::prelude::*;

#[error_code]
pub enum CasinoError {
    #[msg("Caller is not authorized to perform this action")]
    Unauthorized,
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Instruction missing required bump")]
    MissingBump,
}
