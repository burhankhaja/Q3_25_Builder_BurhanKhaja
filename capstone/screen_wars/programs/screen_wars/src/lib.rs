pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

pub mod helpers;

declare_id!("4jqrWDfeR2RAzSPYNoiVq2dcVrZUrsp3ZWEPHehVwCtW");

#[program]
pub mod screen_wars {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_global_account(&ctx.bumps)
    }

    pub fn create_challenge(
        ctx: Context<CreateChallenge>,
        start_time: i64,
        daily_timer: i64,
    ) -> Result<()> {
        ctx.accounts
            .create_new_challenge(start_time, daily_timer, &ctx.bumps)?;
        ctx.accounts.increment_global_challenge_ids()
    }

    pub fn join_challenge(ctx: Context<JoinChallenge>, _challenge_id: u32) -> Result<()> {
        ctx.accounts.validate_challenge_has_not_started()?;
        ctx.accounts
            .initialize_user_account(_challenge_id, &ctx.bumps)?;
        ctx.accounts.increment_total_participants()
    }

    // fn : sync_and_lock
    pub fn withdraw_and_close(ctx: Context<WithdrawClose>, _challenge_id: u32) -> Result<()> {
        ctx.accounts.validate_challenge_has_ended()?;
        ctx.accounts.validate_user_is_enrolled_in_challenge()?;
        ctx.accounts.transfer_sol()
    }

    pub fn claim_winner_position(
        ctx: Context<ClaimWinnerPosition>,
        _challenge_id: u32,
    ) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        // validate
        ctx.accounts.validate_user_is_enrolled_in_challenge()?;
        ctx.accounts.validate_challenge_has_ended(now)?;
        ctx.accounts.validate_reward_claiming_has_not_started(now)?;

        // set
        ctx.accounts.set_winner()
    }

    pub fn claim_rewards(ctx: Context<ClaimRewards>, _challenge_id: u32) -> Result<()> {
        ctx.accounts.validate_caller_is_winner()?;
        ctx.accounts.validate_contention_period_is_over()?;
        ctx.accounts.transfer_sol()
    }

    pub fn take_protocol_profits(ctx: Context<Profit>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_from_treasury(amount)
    }
}
