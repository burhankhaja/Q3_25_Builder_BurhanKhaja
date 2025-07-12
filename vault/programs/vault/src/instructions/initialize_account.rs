use crate::state::UserVault;
use anchor_lang::prelude::*;

// // //  initialize context and state

#[derive(Accounts)]
pub struct InitializeAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + UserVault::INIT_SPACE,
        seeds = [b"vault", signer.key().as_ref()],
        bump,
    )]
    pub user_vault: Account<'info, UserVault>, //will store bump + user's sol

    system_program: Program<'info, System>,
}

impl<'info> InitializeAccount<'info> {
    pub fn initialize(&mut self, bump: u8) -> Result<()> {
        *self.user_vault = UserVault { vault_bump: bump }; //@note try using set_inner instead of dereference
        Ok(())
    }
}
