use anchor_lang::prelude::*;

#[error_code]
pub enum VestingError {
    #[msg("It's not time to release tokens yet.")]
    TooEarly,
    #[msg("All tokens have already been claimed.")]
    AlreadyClaimed,
}