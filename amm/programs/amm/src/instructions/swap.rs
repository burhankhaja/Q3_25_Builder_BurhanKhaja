use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::PoolConfig;

#[derive(Accounts)]
#[instruction(_pool_id: u16)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mint::token_program = token_program)]
    pub mint_x: Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
    pub mint_y: Account<'info, Mint>,

    #[account(
        seeds = [b"pool_config", _pool_id.to_le_bytes().as_ref()],
        bump = pool_config.config_bump,
        has_one = mint_x,  // Ensures mint_x matches the one in config
        has_one = mint_y,  // Ensures mint_y matches the one in config
    )]
    pub pool_config: Account<'info, PoolConfig>,

    #[account(
        seeds = [b"lp",  _pool_id.to_le_bytes().as_ref()],
        bump = pool_config.lp_bump
    )]
    pub mint_lp: Account<'info, Mint>,

    // vault atas
    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = pool_config,
        associated_token::token_program = token_program,
    )]
    pub vault_x: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = pool_config,
        associated_token::token_program = token_program,
    )]
    pub vault_y: Account<'info, TokenAccount>,

    /// Users ata's
    /*@note:  since swap_context doesnt know which tokens users are willing to get that is why init-if-needed is neccessary, cause we dont know if user has ata for that token , it could either be token_x in case !is_x and vice-versa*/

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_x,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_x: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_y,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_ata_y: Account<'info, TokenAccount>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Swap<'info> {
    pub fn swap(
        &mut self,
        _pool_id: u16,
        is_x: bool,
        amount_in: u64,
        min_out: u64,
        deadline: i64,
    ) -> Result<()> {
        //Deadline, pool lock and amount validation

        // curve lib init and get swap result
        // validate swapresult.* > 0

        // @audit :: temporary value until logic updated
        let deposit_amount = 0;
        let withdraw_amount = 0;

        // settle and take tokens
        self.settle(is_x, deposit_amount)?;
        self.take(_pool_id, is_x, withdraw_amount)?;

        Ok(())
    }

    pub fn settle(&mut self, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.user_ata_x.to_account_info(),
                self.vault_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.user_ata_y.to_account_info(),
                self.vault_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked {
            from,
            to,
            authority: self.user.to_account_info(),
            mint,
        };

        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_context, amount, decimals)?;
        Ok(())
    }

    pub fn take(&mut self, _pool_id: u16, is_x: bool, amount: u64) -> Result<()> {
        let (from, to, mint, decimals) = match is_x {
            true => (
                self.vault_x.to_account_info(),
                self.user_ata_x.to_account_info(),
                self.mint_x.to_account_info(),
                self.mint_x.decimals,
            ),
            false => (
                self.vault_y.to_account_info(),
                self.user_ata_y.to_account_info(),
                self.mint_y.to_account_info(),
                self.mint_y.decimals,
            ),
        };

        let cpi_accounts = TransferChecked {
            from,
            to,
            mint,
            authority: self.pool_config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool_config",
            &_pool_id.to_le_bytes(), //@err :: If .as_ref pattern , """temporary value dropped""" error >> why ?
            &[self.pool_config.config_bump],
        ]];

        let cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            cpi_accounts,
            signer_seeds,
        );

        transfer_checked(cpi_context, amount, decimals)?;

        Ok(())
    }
}
