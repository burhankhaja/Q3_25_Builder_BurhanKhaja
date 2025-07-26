use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};

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
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    #[account(
        seeds = [b"global_config"], 
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        seeds = [b"rewards"],
        bump,
        // mint::decimals = 9,
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = reward_mint,
        associated_token::authority = user,

    )]
    user_reward_ata: Account<'info, TokenAccount>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        // Enforcing Checks Effects Interactions Design pattern
        // Cache && nullify users_state.earned_tokens
        let reward_tokens_earned = self.user_account.earned_tokens;

        // validate user has something to mint @dev :: lets use if statement for this instead of require!
        if reward_tokens_earned > 0 {
            return Err(StakeErrors::NoRewards.into());
        }

        // nullify user's reward state
        self.user_account.earned_tokens = 0;

        // mint rewards
        let mint_amount = (reward_tokens_earned as u64)
            .checked_mul(self.reward_mint.decimals as u64)
            .ok_or(StakeErrors::IntegerOverflow)
            .unwrap(); // Be careful while casting, dont try to downcast amounts with precision !!!

        self.mint(mint_amount)
    }

    pub fn mint(&mut self, mint_amount: u64) -> Result<()> {
        let mint_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(), // send to user_ata __init_if_needed ??
            authority: self.global_config.to_account_info(),
        };

        //@later :: Maybe later find some assembly type thing to store seeds sucht that you derive them via wrapping and unwrapping a state via helpers

        let signer_seeds: &[&[&[u8]]] =
            &[&[b"global_config", &self.global_config.bump.to_le_bytes()]]; //@must:: understand this array structury truly /// deeply ,,,

        let mint_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            mint_accounts,
            signer_seeds,
        );

        mint_to(mint_context, mint_amount)?;

        Ok(())
    }
}
