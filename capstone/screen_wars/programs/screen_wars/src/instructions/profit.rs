use crate::{
    error::Errors,
    helpers,
    state::Global,
};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct Profit<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        mut,
        seeds = [b"global"],
        bump = global.bump,
        has_one = admin,
    )]
    pub global: Account<'info, Global>,

    #[account(mut)]
    pub to_optional: Option<AccountInfo<'info>>,

    pub system_program: Program<'info, System>,
}

impl<'info> Profit<'info> {
    pub fn validate_solvency(&mut self, amount: u64) -> Result<()> {
        require!(self.global.treasury_profits >= amount, Errors::OverClaim);
        Ok(())
    }

    pub fn withdraw_from_treasury(&mut self, amount: u64) -> Result<()> {
        let global = &self.global.to_account_info();

        let receiver: &AccountInfo<'info> = match &self.to_optional {
            Some(given_address) => given_address,
            None => &self.admin.to_account_info(),
        };

        helpers::transfer_from_pda(global, receiver, amount)
    }

    pub fn update_treasury_profits(&mut self, amount: u64) -> Result<()> {
        helpers::update_treasury_profits(&mut self.global, false, amount)
    }
}
