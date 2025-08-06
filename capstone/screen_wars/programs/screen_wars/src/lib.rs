pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

pub mod helpers;

use crate::error::Errors;

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
        ctx.accounts.validate_challenge_creation_is_unpaused()?;
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

    pub fn sync_and_lock(ctx: Context<SyncLock>) -> Result<()> {
        let (user_passed_today, days_not_synced) = ctx.accounts.mock_offchain_oracle_component()?;
        let today = 1;

        ctx.accounts
            .deposit_total_daily_lamports(days_not_synced + today)?;

        let mut days_not_synced_or_failed = days_not_synced;

        if !user_passed_today {
            days_not_synced_or_failed += 1; // no need for checked_add
        }

        if days_not_synced_or_failed > 0 {
            ctx.accounts.reset_streak()?;

            let current_balance = ctx.accounts.user_account.locked_balance;
            let lb_penalty = ctx
                .accounts
                .calculate_exponential_penalty_on_locked_balance(
                    current_balance,
                    days_not_synced_or_failed,
                )?;

            // slash
            ctx.accounts
                .update_users_locked_balance(-(lb_penalty as i64))?;

            // total penalty is applied by slashing all the  daily_lamports + 25% of previous locked_balance
            // (SyncLock::DAILY_LAMPORTS * days_not_synced_or_failed) + lb_penalty
            let total_penalty = SyncLock::DAILY_LAMPORTS
                .checked_mul(days_not_synced_or_failed as u64)
                .ok_or(Errors::IntegerOverflow)?
                .checked_add(lb_penalty)
                .ok_or(Errors::IntegerOverflow)?;

            msg!("Total penalty: {:?}", total_penalty);
        }

        if user_passed_today {
            ctx.accounts.increment_streak()?;
            ctx.accounts
                .update_users_locked_balance(SyncLock::DAILY_LAMPORTS as i64)?; // increase
        }

        Ok(())
    }

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
        ctx.accounts.transfer_sol()?;

        let treasury_profit_from_challenge = ctx
            .accounts
            .challenge
            .total_slashed
            .checked_div(2)
            .ok_or(Errors::IntegerUnderflow)?;

        ctx.accounts
            .update_treasury_profits(treasury_profit_from_challenge)
    }

    pub fn take_protocol_profits(ctx: Context<Profit>, amount: u64) -> Result<()> {
        ctx.accounts.validate_solvency(amount)?;
        ctx.accounts.withdraw_from_treasury(amount)?;
        ctx.accounts.update_treasury_profits(amount)
    }

    pub fn set_challenge_creation_paused(
        ctx: Context<ToggleChallengeCreation>,
        pause: bool,
    ) -> Result<()> {
        ctx.accounts.update_challenge_creation_state(pause)
    }
}
