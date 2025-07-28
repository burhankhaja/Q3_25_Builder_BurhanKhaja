use crate::Global;
use anchor_lang::{prelude::*, solana_program::clock};

#[derive(Accounts)]
pub struct UpdateFee<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global"],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,
}

impl<'info> UpdateFee<'info> {
    pub fn update(&mut self, new_fee: u16) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;

        // Admin can re update fee after one week from previous update
        // but for sellers `new_fee`` will  be applicable only after 2 weeks from the update
        let one_week_in_seconds = 604800;
        require!(
            now.checked_sub(self.global.new_fee_at).unwrap() > one_week_in_seconds,
            MarketplaceErrors::FeeUpdateDelay
        );

        // protocol fee can range only between 0 - 0.5%
        require!(new_fee <= 50, MarketplaceErrors::MaxFee);

        self.global.new_fee = new_fee;
        self.global.new_fee_at = now;

        msg!(
            "Fee change from {:?} to {:?} after 2 week period initiated",
            self.global.fee,
            new_fee
        );
        Ok(())
    }
}

#[error_code]
pub enum MarketplaceErrors {
    #[msg("Protocol fee can only be updated upto 0.5% of nft selling price")]
    MaxFee,

    #[msg("Admin can re-update fee after 7 week delay")]
    FeeUpdateDelay,
}
