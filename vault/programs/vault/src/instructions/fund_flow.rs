use crate::state::UserVault;
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

// // // deposit/withdraw context and state
#[derive(Accounts)]
pub struct FundFlow<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", signer.key().as_ref()],
        bump = user_vault.vault_bump,
    )]
    pub user_vault: Account<'info, UserVault>,

    pub system_program: Program<'info, System>,
}

impl<'info> FundFlow<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let transfer_accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.user_vault.to_account_info(),
        };

        let transfer_context =
            CpiContext::new(self.system_program.to_account_info(), transfer_accounts);

        transfer(transfer_context, amount)?;

        Ok(())
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let binding = self.signer.key();
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"vault", binding.as_ref(), &[self.user_vault.vault_bump]]];

        let transfer_accounts = Transfer {
            from: self.user_vault.to_account_info(),
            to: self.signer.to_account_info(),
        };
        let transfer_context = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer(transfer_context, amount)?;

        Ok(())
    }
}
