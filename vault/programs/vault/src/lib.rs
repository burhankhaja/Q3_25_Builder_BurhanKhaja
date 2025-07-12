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

    pub fn withdraw(ctx: Context<FundFlow>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
        // Ok(())
    }

    // close
    pub fn closeAccount(ctx: Context<CloseVaultAccount>) -> Result<()> {
        msg!(
            "Your vault account ` {:?} ` has been closed and all the rent stored has been repaid ",
            ctx.accounts.user_vault.key()
        );

        Ok(())
    }
}
