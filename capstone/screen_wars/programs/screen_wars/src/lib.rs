pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

pub mod helpers;

use crate::{error::Errors, helpers::DebugData};

declare_id!("4jqrWDfeR2RAzSPYNoiVq2dcVrZUrsp3ZWEPHehVwCtW");

#[program]
pub mod screen_wars {
    use super::*;

    /// ====================================
    /// ========= Admin Functions ==========
    /// ====================================

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.initialize_global_account(&ctx.bumps)
    }

    pub fn set_challenge_creation_paused(
        ctx: Context<ToggleChallengeCreation>,
        pause: bool,
    ) -> Result<()> {
        ctx.accounts.update_challenge_creation_state(pause)
    }

    pub fn take_protocol_profits(ctx: Context<Profit>, amount: u64) -> Result<()> {
        ctx.accounts.validate_solvency(amount)?;
        ctx.accounts.withdraw_from_treasury(amount)?;
        helpers::update_treasury_profits(&mut ctx.accounts.global, -(amount as i64))
    }

    /// ====================================
    /// ========= User Functions ===========
    /// ====================================

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

    pub fn sync_and_lock(ctx: Context<SyncLock>, debug: Option<DebugData>) -> Result<()> {
        let (user_passed_today, days_not_synced) = helpers::mock_offchain_oracle_component(debug)?;

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

            // slashed amounts are future rewards
            ctx.accounts
                .update_total_slashed_in_challenge(total_penalty)?;
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

    pub fn claim_rewards_as_winner(ctx: Context<ClaimRewards>, _challenge_id: u32) -> Result<()> {
        ctx.accounts.validate_caller_is_winner()?;
        ctx.accounts.validate_contention_period_is_over()?;

        //// calculate rewards
        let (winner_rewards, _, treasury_profits) = ctx.accounts.calculate_rewards()?;

        //// close challenge account if claimed by creator, otherwise update treasury profits
        let claimed_by_creator = ctx.accounts.challenge.creator_has_claimed;
        if claimed_by_creator {
            ctx.accounts.close_challenge_account()?;
        } else {
            helpers::update_treasury_profits(&mut ctx.accounts.global, treasury_profits as i64)?;
        }

        // winner state is nullified with default pubkey after claiming to prevent fund draining
        ctx.accounts.set_winner_claimed()?;
        ctx.accounts.transfer_sol(winner_rewards)
    }

    pub fn claim_rewards_as_creator(ctx: Context<ClaimRewards>, _challenge_id: u32) -> Result<()> {
        ctx.accounts.validate_caller_is_creator()?;
        ctx.accounts.validate_contention_period_is_over()?;

        //// calculate rewards
        let (_, creator_rewards, treasury_profits) = ctx.accounts.calculate_rewards()?;

        //// close challenge account if claimed by winner, otherwise update treasury profits
        let claimed_by_winner = ctx.accounts.challenge.winner_has_claimed;
        if claimed_by_winner {
            ctx.accounts.close_challenge_account()?;
        } else {
            helpers::update_treasury_profits(&mut ctx.accounts.global, treasury_profits as i64)?;
        }

        // creator state is nullified with default pubkey after claiming to prevent fund draining
        ctx.accounts.set_creator_claimed()?;
        ctx.accounts.transfer_sol(creator_rewards)
    }
}
