use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{transfer_checked, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction( extra_seed: String)]
pub struct MakeOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub offered_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub expected_mint: InterfaceAccount<'info, Mint>,

    // since i have to take tokens from user , pass his ATA
    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_offered_ata: InterfaceAccount<'info, TokenAccount>,

    // create an escrow
    #[account(
        init,
        payer = maker,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", maker.key().as_ref(), extra_seed.as_ref()],
        bump,

    )]
    pub escrow: Account<'info, Escrow>,

    // create an ata for escrow
    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/*  @note :: InterfaceAccount ? Interface ? Program ?

    InterfaceAccount: expects an account implements certain interface
    Interface : expects a program that implements certain interface
    Program: expects a certain well defined Program

*/

impl<'info> MakeOffer<'info> {
    // initialize escrow
    pub fn initialize_escrow(
        &mut self,
        bump: &MakeOfferBumps,
        deposit_amount: u64,
        expect_amount: u64,
    ) -> Result<()> {
        *self.escrow = Escrow {
            maker: self.maker.key(),
            offered_mint: self.offered_mint.key(),
            expected_mint: self.expected_mint.key(),
            offered_amount: deposit_amount,
            expected_amount: expect_amount,
            bump: bump.escrow,
        }; //@note Instead of dereference you can also use self.escrow.set_inner(Escrow{....})

        Ok(())
    }

    // deposit tokens into vault
    pub fn deposit_tokens(&mut self, deposit_amount: u64) -> Result<()> {
        let transfer_accounts = TransferChecked {
            // Transfer tokens from makers offered token ata to vault
            from: self.maker_offered_ata.to_account_info(),
            mint: self.offered_mint.to_account_info(),
            to: self.vault.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        let cpi_context = CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(cpi_context, deposit_amount, self.offered_mint.decimals)?;

        Ok(())
    }
}
