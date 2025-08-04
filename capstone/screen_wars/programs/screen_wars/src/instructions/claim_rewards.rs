use anchor_lang::prelude::*;

use crate::{
    error::Errors,
    helpers::transfer_from_pda,
    state::{Challenge, Global},
};

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"challenge", _challenge_id.to_be_bytes().as_ref() ], 
        bump = challenge.bump,
        close = user,
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        seeds = [b"global"],
        bump = global.bump,
    )]
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimRewards<'info> {
    pub fn validate_caller_is_winner(&mut self) -> Result<()> {
        require!(self.user.key() == self.challenge.winner, Errors::NotWinner);

        Ok(())
    }

    pub fn validate_contention_period_is_over(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let five_days = 5 * 24 * 60 * 60; //@dev :: must ::  later store all helper variables in separate file, and import from that across whole protocol files
        let contention_period = self
            .challenge
            .end
            .checked_add(five_days)
            .ok_or(Errors::IntegerOverflow)
            .unwrap();

        require!(now > contention_period, Errors::ContentionPhase);

        Ok(())
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        let rewards = self
            .challenge
            .total_slashed
            .checked_div(2)
            .ok_or(Errors::IntegerUnderflow)
            .unwrap();

        let global = &self.global.to_account_info();
        let user = &self.user.to_account_info();

        transfer_from_pda(global, user, rewards)
    }
}
