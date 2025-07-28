use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::Offer;

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"listing", seller.key().as_ref(), mint.key().as_ref()],
        bump = listing.bump,
        close = seller,
    )]
    pub listing: Account<'info, Offer>,

    // cpi programs
    pub system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self) -> Result<()> {
        // close listing and withdraw nft back to seller
        self.withdraw_nft()
    }

    pub fn withdraw_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
