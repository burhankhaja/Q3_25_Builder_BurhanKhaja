use crate::{error::Errors, state::Global};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct ToggleChallengeCreation<'info> {
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

impl<'info> ToggleChallengeCreation<'info> {
    pub fn update_challenge_creation_state(&mut self, pause: bool) -> Result<()> {
        require!(
            self.global.challenge_creation_paused != pause,
            Errors::ChallengeStateAlreadySet
        );

        self.global.challenge_creation_paused = pause;

        Ok(())
    }
}
