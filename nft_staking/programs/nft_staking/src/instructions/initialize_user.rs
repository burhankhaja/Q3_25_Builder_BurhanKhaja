use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = UserAccount::DISCRIMINATOR.len() + UserAccount::INIT_SPACE,
        seeds = [b"user_account", user.key().as_ref()],
        bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
}

impl<'info> InitializeUser<'info> {
    pub fn init_user(&mut self) -> Result<()> {
        self.user_account.set_inner(UserAccount {
            earned_tokens: (0),
            staked_amount: (0),
            bump: (self.user_account.bump),
        });

        Ok(())
    }
}
