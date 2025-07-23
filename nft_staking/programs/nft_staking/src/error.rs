use anchor_lang::prelude::*;

#[error_code]
pub enum StakeErrors {
    #[msg("can't stake beyond maximum allowed stake")]
    MaxStake,

    #[msg("Minimum Freeze Period hasn't passed yet")]
    FreezePeriod,

    #[msg("No rewards available to claim.")]
    NoRewards,

    #[msg("Integer Overflow")]
    IntegerOverflow,

    #[msg("Integer Underflow")]
    IntegerUnderflow,
}
