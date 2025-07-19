use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, TokenAccount, Token, transfer_checked,TransferChecked,   mint_to, MintTo, burn,  Burn  },
};
use crate::{PoolConfig}; //? what do we need global for in here ?? no authority thing 

#[derive(Accounts)]
#[instruction(_pool_id: u16)]
pub struct Liquidity<'info> {
 
    #[account(mut)]
    pub user: Signer<'info>,


    /// deposit tokens
    pub mint_x: Account<'info, Mint>,
    pub mint_y: Account<'info, Mint>,

  
    // pool config && lp_mint of pool_id
    #[account(
        seeds = [b"pool_config", _pool_id.to_le_bytes().as_ref()],
        bump = pool_config.config_bump,     
        has_one = mint_x, 
        has_one = mint_y,  
    )]
    pub pool_config: Account<'info, PoolConfig>,

    #[account(
        mut,
        seeds = [b"lp",  _pool_id.to_le_bytes().as_ref()],
        bump = pool_config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,


    // deposit token atas of pool_id
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool_config,
        associated_token::token_program = token_program
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool_config,
        associated_token::token_program = token_program
    )]
    pub vault_y: Account<'info, TokenAccount>,
    

    /// user's deposit tokens and lp_token ATAs
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_lp: Account<'info, TokenAccount>,


    /// Cpi programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Liquidity<'info> {
    
}