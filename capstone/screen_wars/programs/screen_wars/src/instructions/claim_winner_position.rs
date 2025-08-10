use crate::{
    error::Errors,
    state::{Challenge, User},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct ClaimWinnerPosition<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"challenge", _challenge_id.to_le_bytes().as_ref() ], 
        bump = challenge.bump,
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref() ], 
        bump,
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> ClaimWinnerPosition<'info> {
    pub fn validate_challenge_has_ended(&mut self, now: i64) -> Result<()> {
        require!(now > self.challenge.end, Errors::ChallengeNotEnded);
        Ok(())
    }

    pub fn validate_reward_claiming_has_not_started(&mut self, now: i64) -> Result<()> {
        let five_days = 5 * 24 * 60 * 60;
        require!(
            now < self.challenge.end + five_days,
            Errors::ContentionExpired
        );
        Ok(())
    }

    pub fn validate_user_is_enrolled_in_challenge(&mut self) -> Result<()> {
        require!(
            self.user_account.challenge_id == self.challenge.challenge_id,
            Errors::NotEnrolled
        );
        Ok(())
    }

    pub fn set_winner(&mut self) -> Result<()> {
        if self.challenge.winner == Pubkey::default() {
            self.write()
        } else {
            require!(
                self.user_account.streak > self.challenge.winner_streak,
                Errors::LowerStreak
            );
            self.write()
        }
    }

    pub fn write(&mut self) -> Result<()> {
        self.challenge.winner = self.user_account.user;
        self.challenge.winner_streak = self.user_account.streak;
        Ok(())
    }
}
