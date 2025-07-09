pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2xmjzsZGrbhDYqjFyZCxjx4xpGAnexGLwJxmA9Qvn8Yd");

#[program]
pub mod vault {
    use super::*;

    pub fn initVaultAccount(ctx: Context<InitializeAccount>) -> Result<()> {
        *ctx.accounts.user_vault = UserVault {
            vault_bump: ctx.bumps.user_vault,
        };
        Ok(())
    }

    // deposit function
    pub fn deposit(ctx: Context<FundFlow>, amount: u32) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<FundFlow>, amount: u32) -> Result<()> {
        Ok(())
    }

    // close
    pub fn closeAccount(ctx: Context<CloseVaultAccount>) -> Result<()> {
        msg!("Your vault account {:?} has been closed and all the rent stored has been repaid ", ctx.accounts.user_vault.key());
        Ok(())
    }
}



// close Account 
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
    pub user_vault: Account<'info, UserVault>,

    pub system_program: Program<'info, System>,
}




// deposit/withdraw context and state
#[derive(Accounts)]
pub struct FundFlow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = user_vault.vault_bump,
    )]
    pub user_vault: Account<'info, UserVault>, // Now implement deposit function

                                               // Then withdraw function

                                               // && call deposit/withdraw in their respective external functions
}

// initialize context and state

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

#[account]
#[derive(InitSpace)] // use the builtin space calculation method instead of implementing on your own
pub struct UserVault {
    pub vault_bump: u8,
}
