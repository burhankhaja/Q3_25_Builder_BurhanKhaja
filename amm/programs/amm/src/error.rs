use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Invalid Lock authority")]
    InvalidAuthority,

    #[msg("Fees exceeds max ceiling")]
    HighFees,

    #[msg("Pool is already in the expected state, make sure to toggle lock / unlock state correctly")]
    SameLockState,
}
