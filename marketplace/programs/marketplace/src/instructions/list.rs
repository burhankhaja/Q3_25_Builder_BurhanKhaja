use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::Offer;

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        init,
        payer = seller,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"listing", seller.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Offer>,

    // cpi programs
    pub system_program: Program<'info, System>,
}

impl<'info> List<'info> {
    pub fn list(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        // create and initialize offer account of user
        self.listing.set_inner(Offer {
            seller: (*self.seller.key),
            price: (price),
            bump: (bumps.listing),
        });

        // transfer nft to the offer account
        self.deposit_nft()?;

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
