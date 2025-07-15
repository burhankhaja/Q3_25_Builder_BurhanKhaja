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

    //@note if we had used SystemAccount on vault pda ... then we had to close it via close_account cpi with signer seeds being of user_vault since it is the authority and the owner is the system_program ::::: in other words close=owner wouldn't have worked on that
    pub system_program: Program<'info, System>,
}
