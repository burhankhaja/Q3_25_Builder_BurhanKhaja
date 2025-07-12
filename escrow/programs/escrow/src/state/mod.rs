use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)] // so that you can log this entirely via get_offer crate !!!
pub struct Escrow {
    pub maker: Pubkey,
    pub offered_mint: Pubkey,  // Token Mint That user is offering
    pub expected_mint: Pubkey, // Token MInt that user is expecting to get
    pub offered_amount: u64,   // token amounts the maker is offering (offered_mint)
    pub expected_amount: u64,  // token amounts the maker wants in return (expected_mint)
    pub bump: u8,
    // @note  : Maybe later, try storing maker's both ata in order to reduce Cu during taking/cancelling of offer /// dont forget doing this will increase cu for making an offer >>>> Just try it maybe
}
