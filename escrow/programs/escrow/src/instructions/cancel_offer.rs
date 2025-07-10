use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token,
    token_2022::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(extra_seed: String)]
pub struct CancelOffer<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        mut,
        // has_one = maker, // i think you don't need it since escrow derivation is 1:1 related with Signer == maker
        seeds = [b"escrow", maker.key().as_ref(), extra_seed.as_ref()],
        bump = escrow.bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(
        mut,
        associated_token::mint = escrow.offered_mint, //@audit :: any issues ?? since escrow.offered_mint is pubkey && not the InterfaceAccount<'info, Mint>
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    // since tokens will be sent to maker offered ata :: dervie that
    #[account(
        mut,
        associated_token::mint = escrow.offered_mint,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_offered_ata: InterfaceAccount<'info, TokenAccount>,

    // since you can't call to_account_info() during cpi context on escrow.offered_mint which is of type PUbkey
    #[account(
        mint::token_program = token_program
    )]
    pub offered_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CancelOffer<'info> {
    pub fn withdraw_offered_amounts(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            // mint: self.escrow.offered_mint.to_account_info(), //@issue :: cant call to_account_info on type PUbkey
            mint: self.offered_mint.to_account_info(),
            to: self.maker_offered_ata.to_account_info(),
            authority: self.maker.to_account_info(),
        };

        Ok(())
    }

    pub fn close_vault(&mut self) -> Result<()> {
        Ok(())
    }

    // pub fn does_pubkey_doesnt_support_direct_account_info_conversion(&mut self) {
    //     let pubkey_str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    //     let pubkey = pubkey_str.parse::<Pubkey>().unwrap();
    //     pubkey.to_account_info(); // compilation error:
    // }
}
