use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccount, FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
            ThawDelegatedAccountCpi, ThawDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount, SetAndVerifyCollection,
    },
    token::{approve, revoke, Approve, FreezeAccount, Mint, Revoke, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    //@account just used for validation, right here in the context struct
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        mut,
        seeds = [b"stake_account", user.key().as_ref(), mint.key().as_ref()], //@audit :: does order matter in seeds, test in ts with findProgramAddressSync ?
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

    // Cpi Programs
    pub system_program: Program<'info, System>, //@stupid_try :: close without this program
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // validate stake + freeze_period
        //?? validate stake ?? later macro ++ bit need to learn more

        let now = Clock::get()?.unix_timestamp;
        let stake_duration = now
            .checked_sub(self.stake_account.staked_at)
            .ok_or(StakeErrors::IntegerOverflow)
            .unwrap();

        require!(
            stake_duration > self.global_config.freeze_period as i64,
            StakeErrors::FreezePeriod
        ); //@weird_issue: only use crate::StakeError::*// every error instance works without problems expect this one when we do `FreePeriod` directly inspite of `StakeErrors::FreezePeriod`, why ????

        // update users rewards
        let days_staked = stake_duration
            .checked_div(86400)
            .ok_or(StakeErrors::IntegerUnderflow)
            .unwrap();

        // reward_tokens_earned = days_staked * config.reward_tokens_per_day
        let reward_tokens_earned = (self.global_config.reward_tokens_per_day as i64)
            .checked_mul(days_staked)
            .ok_or(StakeErrors::IntegerOverflow)
            .unwrap();

        // Update user's rewards and reduce its stake_amount by 1
        self.user_account
            .earned_tokens
            .checked_add(reward_tokens_earned as u32)
            .ok_or(StakeErrors::IntegerOverflow)
            .unwrap();

        self.user_account
            .staked_amount
            .checked_sub(1)
            .ok_or(StakeErrors::IntegerUnderflow)?;

        // unfreeze user's staked nft
        self.unfreeze_nft()?;

        Ok(())
    }

    pub fn unfreeze_nft(&mut self) -> Result<()> {
        // Unfreeze Nft
        let metadata_program = self.metadata_program.to_account_info();
        let token_program = self.token_program.to_account_info();

        let user_key = self.user.clone().key();
        let mint_key = self.mint.key();

        let signer_seeds: &[&[&[u8]]] =
            &[&[b"stake_account", &user_key.as_ref(), &mint_key.as_ref()]];

        ThawDelegatedAccountCpi::new(
            &metadata_program,
            ThawDelegatedAccountCpiAccounts {
                delegate: &self.stake_account.to_account_info(),
                token_account: &self.user_ata.to_account_info(),
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_program: &token_program,
            },
        )
        .invoke_signed(signer_seeds)?;

        // Revoke Delegation from Stake account
        //@dev :: Lets try non-idiomatic style

        revoke(CpiContext::new(
            token_program,
            Revoke {
                source: self.user_ata.to_account_info(),
                authority: self.user.to_account_info(),
            },
        ))?;
        Ok(())
    }
}
