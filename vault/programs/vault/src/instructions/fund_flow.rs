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

    // #[account(
    //     seeds = [b"vault", user_vault.key().as_ref()],
    //     bump,
    // )]
    // pub vault: SystemAccount<'info>, //@note :::: i haven't used this design, that is why i am caling withdraw_unorthodox instead of system style transfer , since system program doesn't own sols in user_vault ... but in vault only
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

    //@dev :: since we are not using system_program controlled pda , and the pda with sol is program controlled , that is why we have to decrease sol within program itself as the system_program can't access its lamports
    //  Unorthodox withdraw — directly manipulate lamports, rent-safe
    pub fn withdraw_unorthodox(&mut self, amount: u64) -> Result<()> {
        let vault_account_info: &mut AccountInfo = &mut self.user_vault.to_account_info();
        let owner_account_info: &mut AccountInfo = &mut self.signer.to_account_info();

        let vault_lamports_initial = vault_account_info.lamports();
        let owner_lamports_initial = owner_account_info.lamports();

        let minimum_balance = Rent::get()?.minimum_balance(vault_account_info.data_len());

        let surplus = vault_lamports_initial
            .checked_sub(minimum_balance)
            .ok_or(ErrorCode::InsufficientFunds)?;

        require!(surplus >= amount, ErrorCode::InsufficientFunds);

        **owner_account_info.lamports.borrow_mut() = owner_lamports_initial
            .checked_add(amount)
            .ok_or(ErrorCode::Overflow)?;

        **vault_account_info.lamports.borrow_mut() = vault_lamports_initial
            .checked_sub(amount)
            .ok_or(ErrorCode::Overflow)?;

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

        transfer(transfer_context, amount)?; //@audit-issue :: since program is the owner of pda, how come you expect system program to complete the transfers

        Ok(())
    }
}
#[error_code]
pub enum ErrorCode {
    #[msg("Not enough balance in the vault.")]
    InsufficientFunds,

    #[msg("Overflow occurred while calculating transfer.")]
    Overflow,
}
