use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace, Debug)] // so that you can log this entirely via get_offer crate !!!
pub struct Escrow {
    pub maker: Pubkey,
    pub mint_a: Pubkey,       // Token Mint That user is offering
    pub mint_b: Pubkey,       // Token MInt that user is expecting to get
    pub offered_amount: u64,  // token amounts the maker is offering (mint_a)
    pub expected_amount: u64, // token amounts the maker wants in return (mint_b)
    pub bump: u8,
    // @note  : Maybe later, try storing maker's both ata in order to reduce Cu during taking/cancelling of offer /// dont forget doing this will increase cu for making an offer >>>> Just try it maybe
}
