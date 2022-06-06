use anchor_lang::prelude::*;

#[error_code]
pub enum CasinoError {
    #[msg("Caller is not authorized to perform this action")]
    Unauthorized,
    #[msg("Numerical Overflow")]
    NumericalOverflow,
    #[msg("Instruction missing required bump")]
    MissingBump,
    #[msg("Ratio divisor cannot be 0")]
    InvalidRatioDiv,
    #[msg("Requested to build deck with too many sets of cards")]
    TooManyDecks,
}
