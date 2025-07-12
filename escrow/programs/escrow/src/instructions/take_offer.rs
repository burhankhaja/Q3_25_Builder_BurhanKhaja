use std::io::Take;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::{close_account, transfer_checked, CloseAccount, TransferChecked},
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::Escrow;

#[derive(Accounts)]
#[instruction(_extra_seed: String,)]
pub struct TakeOffer<'info> {
    //@note :: you will need makers_ata_b, vault  &&&& taker_ata_a, taker_ata_b
    #[account(mut)]
    pub taker: Signer<'info>,

    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_a: InterfaceAccount<'info, TokenAccount>, //@audit :: try using Account<'info,T> instead !

    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = taker,
        associated_token::token_program = token_program,
    )]
    pub taker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // maker
    #[account(mut)]
    pub maker: SystemAccount<'info>,
    // pub maker: Account<'info, SystemAccount>, // Error ::: check bottome of page with error_0

    // maker offered mint .............. receiving token for the taker
    #[account(
        mint::token_program = token_program
    )]
    pub mint_a: InterfaceAccount<'info, Mint>,

    // maker expected mint
    #[account(
        mint::token_program = token_program
    )]
    pub mint_b: InterfaceAccount<'info, Mint>,

    // maker escrow
    #[account(
        mut,
        seeds = [b"escrow", maker.key().as_ref(), _extra_seed.as_ref()],
        bump = escrow.bump,
        close = maker,
    )]
    pub escrow: Account<'info, Escrow>,

    // maker vault == ata of escrow --(authorized by maker himself)-- that holds makers offered tokens
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>, //@audit :: change whole project naming convention:: vault == maker_offered_token_ata // also name offered_mint to === maker_offered_mint in all functions

    // makers expected token ata
    #[account(
        mut,
        associated_token::mint = mint_b,
        associated_token::authority = maker,
        associated_token::token_program = token_program,
    )]
    pub maker_ata_b: InterfaceAccount<'info, TokenAccount>,

    // ata_program , system_program, token_program
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> TakeOffer<'info> {
    // transfer mint_b tokens from taker's ata to the maker's ata
    pub fn deposit(&mut self) -> Result<()> {
        let transfer_accounts = TransferChecked {
            from: self.taker_ata_b.to_account_info(),
            mint: self.mint_b.to_account_info(),
            to: self.maker_ata_b.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let transfer_context =
            CpiContext::new(self.token_program.to_account_info(), transfer_accounts);

        transfer_checked(
            transfer_context,
            self.escrow.expected_amount,
            self.mint_b.decimals,
        )?;

        Ok(())
    }

    //@note@dev :: one of the benefits of doing both close and withdraw operation in single function is that you get to derive signer seeds only once, hence lesser the codesize and more the readability
    pub fn withdraw_and_close_vault(&mut self, _extra_seed: String) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"escrow",
            self.escrow.maker.as_ref(),
            _extra_seed.as_ref(),
            &[self.escrow.bump], //@audit :: &self.escrow.bump.to_le_bytes() works the same ??? test if diff ???
        ]];

        // // withdraw mint_a tokens from vault and send them to the taker  ata

        let transfer_accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.mint_a.to_account_info(),
            to: self.taker_ata_a.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let transfer_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            transfer_accounts,
            signer_seeds,
        );

        transfer_checked(
            transfer_context,
            self.vault.amount,
            // self.escrow.offered_amount,  // @note :: same thing since we correctly updated this state during making offer
            self.mint_b.decimals,
        )?;

        // // // Now close vault

        let close_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.maker.to_account_info(),
            authority: self.escrow.to_account_info(),
        };

        let close_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            close_accounts,
            signer_seeds,
        );

        close_account(close_context)?;

        Ok(())
    }
}

/*
## error_0
```
the trait bound `anchor_lang::prelude::SystemAccount<'info>: anchor_lang::AccountSerialize` is not satisfied
the following other types implement trait `anchor_lang::AccountSerialize`:
  UpgradeableLoaderState
  __idl::IdlAccount
  anchor_lang::ProgramData
  anchor_lang::idl::IdlAccount
  anchor_spl::token::Mint
  anchor_spl::token::TokenAccount
  anchor_spl::token_interface::Mint
  anchor_spl::token_interface::TokenAccount
  state::EscrowrustcClick for full compiler diagnostic
```


*/
