use crate::{
    error::Errors,
    state::{Challenge, Global},
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct CreateChallenge<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global"],
        bump = global.bump,

    )]
    pub global: Account<'info, Global>,

    #[account(
        init,
        payer = creator,
        space = Challenge::DISCRIMINATOR.len() + Challenge::INIT_SPACE,
        seeds = [b"challenge", global.challenge_ids.to_le_bytes().as_ref()], 
        bump,
    )]
    pub challenge: Account<'info, Challenge>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateChallenge<'info> {
    pub fn validate_challenge_creation_is_unpaused(&mut self) -> Result<()> {
        require!(
            !self.global.challenge_creation_paused,
            Errors::ChallengeCreationPaused
        );
        Ok(())
    }

    pub fn create_new_challenge(
        &mut self,
        start_time: i64,
        daily_timer: i64,
        bumps: &CreateChallengeBumps,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        let two_hours = 2 * (60 * 60);
        let one_day = two_hours * 12;
        let one_week = one_day * 7;
        let three_weeks = one_week * 3;

        require!(start_time > now + one_day, Errors::ChallengeStartsTooSoon);
        require!(start_time < now + one_week, Errors::ChallengeStartsTooFar);
        require!(daily_timer < two_hours, Errors::ChallengeExceedsTwoHours);

        let end_time = start_time
            .checked_add(three_weeks)
            .ok_or(Errors::IntegerOverflow)?;

        self.challenge.set_inner(Challenge {
            creator: *self.creator.key,
            challenge_id: self.global.challenge_ids,
            daily_timer,
            start: start_time,
            end: end_time,
            total_slashed: 0,
            winner: Pubkey::default(),
            winner_streak: 0,
            winner_has_claimed: false,
            creator_has_claimed: false,
            total_participants: 0,
            bump: bumps.challenge,
        });

        Ok(())
    }

    pub fn increment_global_challenge_ids(&mut self) -> Result<()> {
        self.global
            .challenge_ids
            .checked_add(1)
            .ok_or(Errors::IntegerOverflow)?; //@audit :: check whether you need to explicity assign value to the pda , test it out whether ids got updated ??
        Ok(())
    }
}
