pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::{prelude::*, system_program};

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2xmjzsZGrbhDYqjFyZCxjx4xpGAnexGLwJxmA9Qvn8Yd");

#[program]
pub mod vault {
    use super::*;

    pub fn init_vault_account(ctx: Context<InitializeAccount>) -> Result<()> {
        ctx.accounts.initialize(ctx.bumps.user_vault)?;

        msg!(
            "Your vault account {:?} has been successfully initialized",
            ctx.accounts.user_vault.key()
        );

        Ok(())
    }

    // deposit function
    pub fn deposit(ctx: Context<FundFlow>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
        // Ok(())
    }

    /*
    The reason @shrinath used SystemAccount<PDA> is like using pda but transferring its ownership to system program
    such that system program can easily modify its sol balance in and out without any ownership issues
     */
    pub fn withdraw(ctx: Context<FundFlow>, amount: u64) -> Result<()> {
        // ctx.accounts.withdraw(amount) //@audit-issue problemetic ;:: since system doenst own pda //// best to use if you had separate pda for storing vault and its ownership was transferred to systemProgram

        ctx.accounts.withdraw_unorthodox(amount)
    }

    // close
    pub fn close_account(ctx: Context<CloseVaultAccount>) -> Result<()> {
        msg!(
            "Your vault account ` {:?} ` has been closed and all the rent stored has been repaid ",
            ctx.accounts.user_vault.key()
        );

        Ok(())
    }
}
