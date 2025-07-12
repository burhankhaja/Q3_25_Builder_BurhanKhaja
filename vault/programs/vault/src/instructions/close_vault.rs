use crate::state::UserVault;
use anchor_lang::prelude::*;

// // // close Account
#[derive(Accounts)]
pub struct CloseVaultAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = user_vault.vault_bump,
        close = signer,
    )]
    pub user_vault: Account<'info, UserVault>, //@audit :: MUST :: separate design decision from cohert:: since cohert uses 2 diff accounts for state and sol deposit>>> test if pda work fine with sol later in ts??

    pub system_program: Program<'info, System>,
}
