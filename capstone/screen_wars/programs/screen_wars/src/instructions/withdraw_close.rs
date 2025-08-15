use anchor_lang::prelude::*;

use crate::{
    error::Errors,
    helpers::transfer_from_pda,
    state::{Challenge, Global, User},
};

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct WithdrawClose<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"global"],
        bump = global.bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        seeds = [b"challenge", _challenge_id.to_le_bytes().as_ref() ], 
        bump = challenge.bump,
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref() ], 
        bump = user_account.bump,
        close = user,
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawClose<'info> {
    pub fn validate_contention_period_is_over(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let five_days = 5 * 24 * 60 * 60; //@dev :: must ::  later store all helper variables in separate file, and import from that across whole protocol files
        let contention_period = self
            .challenge
            .end
            .checked_add(five_days)
            .ok_or(Errors::IntegerOverflow)?;

        require!(now > contention_period, Errors::ContentionPhase);

        Ok(())
    }

    pub fn validate_user_is_enrolled_in_challenge(&mut self) -> Result<()> {
        require!(
            self.user_account.challenge_id == self.challenge.challenge_id,
            Errors::NotEnrolled
        );

        Ok(())
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        let locked_balance = self.user_account.locked_balance;

        if locked_balance > 0 {
            let global = &self.global.to_account_info();
            let user = &self.user.to_account_info();

            transfer_from_pda(global, user, locked_balance)?;
        }

        Ok(())
    }
}
