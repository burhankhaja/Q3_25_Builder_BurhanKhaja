use crate::{
    error::Errors,
    state::{Challenge, Global, User},
};

use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct JoinChallenge<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"global"],
        bump = global.bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        mut,
        seeds = [b"challenge", _challenge_id.to_le_bytes().as_ref() ], 
        bump = challenge.bump,
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        init,
        payer = user,
        space = User::DISCRIMINATOR.len() + User::INIT_SPACE,
        seeds = [b"user", user.key().as_ref() ], 
        bump,
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> JoinChallenge<'info> {
    pub fn validate_challenge_has_not_started(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(now < self.challenge.start, Errors::JoinedLate);
        Ok(())
    }

    pub fn initialize_user_account(
        &mut self,
        _challenge_id: u32,
        bumps: &JoinChallengeBumps,
    ) -> Result<()> {
        // validate
        // require now < self.challenge.start;
        // require now < self.challenge.end; // redundant check

        // set user state
        self.user_account.set_inner(User {
            user: *self.user.key,
            challenge_id: _challenge_id,
            locked_balance: 0,
            streak: 0,
            bump: bumps.user_account,
        });
        Ok(())
    }

    pub fn increment_total_participants(&mut self) -> Result<()> {
        self.challenge.total_participants = self
            .challenge
            .total_participants
            .checked_add(1)
            .ok_or(Errors::IntegerOverflow)?;

        Ok(())
    }
}
