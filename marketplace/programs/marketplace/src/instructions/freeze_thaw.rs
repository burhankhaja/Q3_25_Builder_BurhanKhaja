use crate::{error::MarketplaceErrors, Global};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct FreezeThaw<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global"],
        bump = global.bump,
        has_one = admin,
    )]
    pub global: Account<'info, Global>,
}

impl<'info> FreezeThaw<'info> {
    pub fn set(&mut self, freeze: bool) -> Result<()> {
        require!(self.global.frozen != freeze, MarketplaceErrors::SameState);

        if freeze {
            self.global.frozen_at = Clock::get()?.unix_timestamp;
        } else {
            self.global.frozen_at = 0;
        }

        self.global.frozen = freeze;

        Ok(())
    }
}
