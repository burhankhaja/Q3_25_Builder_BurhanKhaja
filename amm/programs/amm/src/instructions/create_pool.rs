use crate::{error::ErrorCode, PoolConfig};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

#[derive(Accounts)]
#[instruction(_pool_id: u16)]

pub struct CreatePool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
     init,
     payer = user,
     space = PoolConfig::DISCRIMINATOR.len() + PoolConfig::INIT_SPACE,
     seeds = [b"pool_config", _pool_id.to_le_bytes().as_ref()], //@try:: &_pool_id.to_le_bytes() instead
     bump,
   )]
    pub pool_config: Account<'info, PoolConfig>,

    // lets restrict token program
    #[account(mint::token_program = token_program)]
    pub mint_x: Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_y: Account<'info, Mint>,

    // program owned ata vault for both mints
    #[account(
        init,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = pool_config, // what if we set authority to global // since no one can mess up program logic regardless but that would require me to pass extra accounts during ata signing logic , so its better to separate that logic
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = pool_config
    )]
    pub vault_y: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = user,
        seeds = [b"lp", _pool_id.to_le_bytes().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = pool_config,
    )]
    pub mint_lp: Account<'info, Mint>,

    /// universal accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CreatePool<'info> {
    pub fn create_pool(&mut self, _pool_id: u16, fee: u16, bumps: &CreatePoolBumps) -> Result<()> {
        //@dev lets only allow maximum of 0.9% fee, note that pool can have 0% fee rate
        require!(fee <= 90, ErrorCode::HighFees);

        *self.pool_config = PoolConfig {
            pool_id: _pool_id,
            mint_x: self.mint_x.key(),
            mint_y: self.mint_y.key(),
            fee: fee,
            locked: false,
            config_bump: bumps.pool_config,
            lp_bump: bumps.mint_lp,
        };
        Ok(())
    }
}
