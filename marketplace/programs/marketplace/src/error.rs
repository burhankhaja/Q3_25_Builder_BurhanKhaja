use anchor_lang::prelude::*;

#[error_code]
pub enum MarketplaceErrors {
    #[msg("Protocol fee can only be updated upto 0.5% of nft selling price")]
    MaxFee,

    #[msg("Admin can re-update fee after 7 week delay")]
    FeeUpdateDelay,

    #[msg("Protocol is in same state")]
    SameState,

    #[msg("Protocol is in frozen state")]
    ProtocolFrozen,

    #[msg("during frozen protocol, Delisting is allowed after 1 week delay")]
    FrozenDelistDelay,
}
