use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{transfer_checked, Mint, Token, TokenAccount, TransferChecked},
};

use crate::{error::ErrorCode, PoolConfig};
use constant_product_curve::{ConstantProduct, LiquidityPair};

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
        has_one = mint_x,
        has_one = mint_y,
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
        require!(
            Clock::get()?.unix_timestamp <= deadline,
            ErrorCode::ExpiredTx
        );
        require!(!self.pool_config.locked, ErrorCode::LockedPoolId);
        require!(amount_in > 0, ErrorCode::InvalidAmount);

        // curve lib init and get swap result
        let mut curve = ConstantProduct::init(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            self.pool_config.fee,
            Some(self.mint_lp.decimals),
        )
        .unwrap(); //@note :: technically you could have used last param as None , since precison set to None defaults to 1e6
                   //@dev :: do some error mapping later

        //@audit-issue :: infl attacks , add fix via state tracking ?

        /// resultant pair will be swapped for the other
        let pair = match is_x {
            true => LiquidityPair::X,
            false => LiquidityPair::Y,
        };

        let swap_result = curve.swap(pair, amount_in, min_out).unwrap(); //@audit :: temporary unwarp() fix :: later map_errors properly

        let deposit_amount = swap_result.deposit;
        let withdraw_amount = swap_result.withdraw;

        require!(
            deposit_amount > 0 && withdraw_amount > 0,
            ErrorCode::InvalidAmount
        );

        //@dev::later ::: handle fee logic ::: make sure to take fee as commissions ??

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
