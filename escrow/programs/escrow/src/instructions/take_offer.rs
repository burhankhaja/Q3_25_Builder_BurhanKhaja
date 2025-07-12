use std::io::Take;

use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
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
        bump = maker_escrow.bump,
        close = maker,
    )]
    pub maker_escrow: Account<'info, Escrow>,

    // maker vault == ata of escrow --(authorized by maker himself)-- that holds makers offered tokens
    #[account(
        mut,
        associated_token::mint = mint_a,
        associated_token::authority = maker_escrow,
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
    pub fn deposit() -> Result<()> {
        // transfer tokenb from taker to the

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
