use crate::{Global, error::MarketplaceErrors};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + Global::INIT_SPACE,
        seeds = [b"global"],
        bump,
    )]
    pub global: Account<'info, Global>,

    // cpi programs
    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize(
        &mut self,
        treasury: Pubkey,
        fee: u16,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        // protocol fee can range only between 0 - 0.5%
        require!(fee <= 50, MarketplaceErrors::MaxFee);

        self.global.set_inner(Global {
            admin: (*self.admin.key),
            treasury: (treasury),
            fee: (fee),
            new_fee: (0),
            new_fee_at: (0),
            frozen: (false),
            frozen_at: (0),
            bump: (bumps.global),
        });
        Ok(())
    }
}
