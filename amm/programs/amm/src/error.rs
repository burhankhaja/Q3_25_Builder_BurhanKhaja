use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Invalid Lock authority")]
    InvalidAuthority,

    #[msg("given amounts are not accepted")]
    InvalidAmount,

    #[msg("Fees exceeds max ceiling")]
    HighFees,

    #[msg(
        "Pool is already in the expected state, make sure to toggle lock / unlock state correctly"
    )]
    SameLockState,

    #[msg("pool is in locked stated")]
    LockedPoolId,

    #[msg("the deadline for transaction has expired")]
    ExpiredTx,

    #[msg("Amounts dont respect expected slippage ")]
    BrokenSlippage,
}
