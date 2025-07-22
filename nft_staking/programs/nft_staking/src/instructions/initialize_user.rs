use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> InitializeUser<'info> {
    pub fn init_user(&mut self) -> Result<()> {
        Ok(())
    }
}
