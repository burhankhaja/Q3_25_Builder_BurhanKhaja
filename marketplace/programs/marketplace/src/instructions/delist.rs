use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

use crate::{error::MarketplaceErrors, Global, Offer};

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

    #[account(
        seeds = [b"global"],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    // cpi programs
    pub system_program: Program<'info, System>,
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self) -> Result<()> {

        // Only delisting is allowed when the protocol is in frozen state but with 1 week delay
        if self.global.frozen {
            let now = Clock::get()?.unix_timestamp;
            const ONE_WEEK : i64 = 7 * 24 * 60 * 60; // 604,800 seconds
            require!(now - self.global.frozen_at >= ONE_WEEK, MarketplaceErrors::FrozenDelistDelay);
        }
        // close listing and withdraw nft back to seller
        self.withdraw_nft()
    }

    pub fn withdraw_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
