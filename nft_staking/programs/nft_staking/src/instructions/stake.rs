use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // temp mint , later user metadata crate ??
    pub mint: Account<'info, Mint>,

    #[account(
        seeds = [b"global_config"], 
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    ///@audit :: lets say CyberPunk nft, how do you determine which id is being staked, In solana unlike ETH, do different mint address represent ids of same collection.... ??? if yes then add mint.key as seed with some string data

    #[account(
        init,
        payer = user,
        space = StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", user.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self) -> Result<()> {
        // do some validations, not max_stake, maybe collection validation(or can be done via constraints)
        require!(
            self.user_account.staked_amount <= self.global_config.max_stake,
            StakeErrors::MaxStake
        );

        //?! @audit collection validation ??

        // freeze user's staked nft
        self.freeze_nft();

        // update stake_account's state
        self.stake_account.set_inner(StakeAccount {
            owner: *self.user.key,
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: self.stake_account.bump,
        });

        // update user_account.staked_amount state
        self.user_account
            .staked_amount
            .checked_add(1)
            .ok_or(StakeErrors::IntegerOverflow)?;

        Ok(())
    }

    pub fn freeze_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
