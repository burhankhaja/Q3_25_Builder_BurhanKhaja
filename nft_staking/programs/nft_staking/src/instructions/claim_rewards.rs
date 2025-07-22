use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref(), mint.key().as_ref()], 
        bump,
        close = user,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    #[account(
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"global_config"], 
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        seeds = [b"rewards"],
        bump,
        // mint::decimals = 9,
    )]
    pub reward_mint: Account<'info, Mint>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        // Enforcing Checks Effects Interactions Design pattern
        // Cache && nullify users_state.earned_tokens
        let reward_tokens_earned = self.user_account.earned_tokens;
        self.user_account.earned_tokens = 0;

        // mint rewards
        let mint_amount = (reward_tokens_earned as u64)
            .checked_mul(self.reward_mint.decimals as u64)
            .ok_or(StakeErrors::IntegerOverflow)
            .unwrap(); // Be careful while casting, dont try to downcast amounts with precision !!!

        self.mint(mint_amount)
    }

    pub fn mint(&mut self, mint_amount: u64) -> Result<()> {
        Ok(())
    }
}
