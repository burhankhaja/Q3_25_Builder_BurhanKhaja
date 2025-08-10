use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

use crate::{
    error::Errors,
    state::{Challenge, Global, User},
};

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct SyncLock<'info> {
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
        mut,
        seeds = [b"user", user.key().as_ref() ], 
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

impl<'info> SyncLock<'info> {
    // On YTD average, 10 million lamports are arount $1 - $2
    pub const DAILY_LAMPORTS: u64 = 10_000_000;

    pub fn deposit_total_daily_lamports(&mut self, days_to_update: u8) -> Result<()> {
        // days_to_update * SyncLock::DAILY_LAMPORTS
        let lamports = (days_to_update as u64)
            .checked_mul(SyncLock::DAILY_LAMPORTS)
            .ok_or(Errors::IntegerOverflow)?;

        transfer(
            CpiContext::new(
                self.system_program.to_account_info(),
                Transfer {
                    from: self.user.to_account_info(),
                    to: self.global.to_account_info(),
                },
            ),
            lamports,
        )
    }

    /// Adjusts the user's locked balance by `amount`, which can be positive (credit) or negative (slash).
    pub fn update_users_locked_balance(&mut self, amount: i64) -> Result<()> {
        let new_balance = (self.user_account.locked_balance as i64)
            .checked_add(amount)
            .ok_or(Errors::IntegerBoundsExceeded)?;

        self.user_account.locked_balance = if new_balance >= 0 {
            new_balance as u64
        } else {
            -(new_balance) as u64
        };

        Ok(())
    }

    pub fn calculate_exponential_penalty_on_locked_balance(
        &mut self,
        current_balance: u64,
        days_not_synced_or_failed: u8,
    ) -> Result<u64> {
        if current_balance == 0 {
            Ok(0)
        } else {
            const SCALE: u128 = 1_000_000;
            const RATE_75_PERCENT: u128 = 750_000;

            // Apply compounding: (RATE^days) / (SCALE^days)

            // RATE_75_PERCENT ^ days_not_synced_or_failed
            let numerator = RATE_75_PERCENT
                .checked_pow(days_not_synced_or_failed as u32)
                .ok_or(Errors::IntegerOverflow)?;

            // SCALE ^ days_not_synced_or_failed
            let denominator = SCALE
                .checked_pow(days_not_synced_or_failed as u32)
                .ok_or(Errors::IntegerOverflow)?;

            // numerator * SCALE / denominator
            let multiplier = numerator
                .checked_mul(SCALE)
                .ok_or(Errors::IntegerOverflow)?
                .checked_div(denominator)
                .ok_or(Errors::IntegerUnderflow)?; // bring back to 1x SCALE

            let balance_u128 = current_balance as u128;

            // balance_u128 * multiplier / SCALE
            let final_balance = balance_u128
                .checked_mul(multiplier)
                .ok_or(Errors::IntegerOverflow)?
                .checked_div(SCALE)
                .ok_or(Errors::IntegerUnderflow)?;

            // balance_u128 - final_balance
            let penalty = balance_u128
                .checked_sub(final_balance)
                .ok_or(Errors::IntegerUnderflow)?;

            Ok(penalty as u64) // => safe downcast, since unscaled amounts
        }
    }

    pub fn reset_streak(&mut self) -> Result<()> {
        self.user_account.streak = 0;
        Ok(())
    }

    pub fn increment_streak(&mut self) -> Result<()> {
        self.user_account.streak += 1; // no need for checked_add , impossible to overflow !
        Ok(())
    }

    pub fn update_total_slashed_in_challenge(&mut self, amount: u64) -> Result<()> {
        self.challenge.total_slashed = self
            .challenge
            .total_slashed
            .checked_add(amount)
            .ok_or(Errors::IntegerOverflow)?;

        Ok(())
    }
}
