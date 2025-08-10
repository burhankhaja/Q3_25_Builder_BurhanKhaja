use anchor_lang::prelude::*;

use crate::{
    error::Errors,
    helpers,
    state::{Challenge, Global},
};

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct ClaimRewards<'info> {
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

    pub fn validate_caller_is_creator(&mut self) -> Result<()> {
        require!(
            self.user.key() == self.challenge.creator,
            Errors::NotCreator
        );

        Ok(())
    }

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

    pub fn transfer_sol(&mut self, reward: u64) -> Result<()> {
        let global = &self.global.to_account_info();
        let user = &self.user.to_account_info();

        helpers::transfer_from_pda(global, user, reward)
    }

    // set claimed function
    pub fn set_winner_claimed(&mut self) -> Result<()> {
        self.challenge.winner_has_claimed = true;
        self.challenge.winner = Pubkey::default();

        Ok(())
    }

    pub fn set_creator_claimed(&mut self) -> Result<()> {
        self.challenge.creator_has_claimed = true;
        self.challenge.creator = Pubkey::default();
        Ok(())
    }

    /// @notice Splits the total slashed amount into rewards: 50% to the winner, 10% to the challenge creator, and the remaining 40% to the protocol.
    /// @dev Uses fixed-point scaling (SCALE = 1_000_000) to simulate decimal division using integers.
    /*
    @dev :: Scaling explanation

    In normal math, to calculate 33% of 98:
        98 * (33 / 100) = 32.34

    But in integer math, (33 / 100) becomes 0, so the result is wrong.

    To fix this, we scale 33 by a factor (e.g. 100):
        98 * ((33 * 100) / 100)

    But now the result is still too large due to the extra scale,
    so we divide once more to bring it back down:
        (98 * ((33 * 100) / 100)) / 100

    In our case, we use SCALE = 1_000_000 for higher precision.
    So, we apply the same logic with large scaled values.

    So that means using 1e6 scaling,
    33% of 98 will become:
        (98 * ((33 * 1_000_000) / 1_000_000)) / 1_000_000
    */
    pub fn calculate_rewards(&mut self) -> Result<(u64, u64, u64)> {
        const SCALE: u128 = 1_000_000;
        const SCALED_50_PERCENT: u128 = 500_000;
        const SCALED_10_PERCENT: u128 = 100_000;

        let total_slashed = self.challenge.total_slashed as u128;

        // (total_slashed * ((SCALED_50_PERCENT * SCALE) / SCALE)) / SCALE;
        let winner_rewards = SCALED_50_PERCENT
            .checked_mul(SCALE)
            .ok_or(Errors::IntegerOverflow)?
            .checked_div(SCALE)
            .ok_or(Errors::IntegerUnderflow)?
            .checked_mul(total_slashed)
            .ok_or(Errors::IntegerOverflow)?
            .checked_div(SCALE)
            .ok_or(Errors::IntegerUnderflow)?;

        // (total_slashed * ((SCALED_10_PERCENT * SCALE) / SCALE)) / SCALE;
        let creator_reward = SCALED_10_PERCENT
            .checked_mul(SCALE)
            .ok_or(Errors::IntegerOverflow)?
            .checked_div(SCALE)
            .ok_or(Errors::IntegerUnderflow)?
            .checked_mul(total_slashed)
            .ok_or(Errors::IntegerOverflow)?
            .checked_div(SCALE)
            .ok_or(Errors::IntegerUnderflow)?;

        let non_protocol_rewards = winner_rewards
            .checked_add(creator_reward)
            .ok_or(Errors::IntegerOverflow)?;

        let protocol_profits = total_slashed
            .checked_sub(non_protocol_rewards)
            .ok_or(Errors::IntegerUnderflow)?;

        Ok((
            winner_rewards as u64,
            creator_reward as u64,
            protocol_profits as u64,
        ))
    }

    /// @dev In reward claim logic, we don't check for `CLOSED_ACCOUNT_DISCRIMINATOR` because we already
    ///      invalidate eligible accounts by setting their Pubkey to `Pubkey::default()`.
    ///      This acts as a soft deletion and prevents re-use.
    pub fn close_challenge_account(&mut self) -> Result<()> {
        let challenge_pda = &self.challenge.to_account_info();
        let rent_receiver = &self.user.to_account_info();

        helpers::close_pda(challenge_pda, rent_receiver)
    }
}
