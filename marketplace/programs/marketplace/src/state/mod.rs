use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Global {
    pub admin: Pubkey,
    pub treasury: Pubkey, //sol address where protocol fee is stored
    pub fee: u16,         // Current protocol fee on purchases
    pub new_fee: u16,     // Updated fee by admin , enforced after 14 day delay
    pub new_fee_at: i64,  // time when fee updated, after two weeks from which new_fee is applicable
    pub frozen: bool,
    pub frozen_at: i64,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Offer {
    pub seller: Pubkey,
    pub price: u64,
    pub bump: u8,
}

// user sell nft --->
