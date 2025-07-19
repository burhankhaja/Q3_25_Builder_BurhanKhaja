use crate::PoolConfig;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        burn, mint_to, transfer_checked, Burn, Mint, MintTo, Token, TokenAccount, TransferChecked,
    },
};

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
    pub fn deposit(&mut self, _pool_id: u16, mint_lp_amount: u64) -> Result<()> {
        // validate key states, locked , amount , deadline, slippage

        /////@later :: add security_first logic for first depositor edge case with normal flow case too

        // take tokens from user and deposit them to vault atas
        let amount_x = 0; //@audit:: temporary value until i add correct logic for different edge cases
        let amount_y = 0;

        self.transfer_from_user(amount_x, amount_y)?;

        // mint lp_amounts to user_lp_ata
        self.mint(_pool_id, mint_lp_amount)?;

        Ok(())
    }

    pub fn transfer_from_user(&mut self, amount_x: u64, amount_y: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let tranfer_x_cpi_accounts = TransferChecked {
            from: self.user_ata_x.to_account_info(),
            to: self.vault_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let tranfer_y_cpi_accounts = TransferChecked {
            from: self.user_ata_y.to_account_info(),
            to: self.vault_y.to_account_info(),
            mint: self.mint_y.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let transfer_x_cpi_context = CpiContext::new(cpi_program.clone(), tranfer_x_cpi_accounts);

        let transfer_y_cpi_context = CpiContext::new(cpi_program.clone(), tranfer_y_cpi_accounts); //@note :: if cpi_program isn't cloned you will get Error "Use of moved value"

        transfer_checked(transfer_x_cpi_context, amount_x, self.mint_x.decimals)?;
        transfer_checked(transfer_y_cpi_context, amount_y, self.mint_y.decimals)?;

        Ok(())
    }

    pub fn mint(&mut self, _pool_id: u16, mint_lp_amount: u64) -> Result<()> {
        // Store pool_id locally so that it lives long enough and prevents "Temporary value dropped" errors
        let pool_id = _pool_id.to_le_bytes();

        let cpi_program = self.token_program.to_account_info();

        let mint_cpi_accounts = MintTo {
            mint: self.mint_lp.to_account_info(),
            to: self.user_ata_lp.to_account_info(),
            authority: self.pool_config.to_account_info(),
        };

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool_config",
            pool_id.as_ref(),
            &[self.pool_config.config_bump],
        ]];

        let mint_cpi_context =
            CpiContext::new_with_signer(cpi_program, mint_cpi_accounts, signer_seeds);

        mint_to(mint_cpi_context, mint_lp_amount)?;

        Ok(())
    }

    ///// withdraw user
    ///
    ///
    ///
    ///
    /// @todo:: pass signer seed from withdraw to transfer_to_user && burn as parameters
    pub fn withdraw(&mut self, _pool_id: u16, burn_lp_amount: u64) -> Result<()> {
        // validate key states, lock, amounts, slippage, expiration,  first deposit vs normal deposit cases

        //@later add security_first fix of withdraw logic

        let pool_id = _pool_id.to_be_bytes();

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool_config",
            &pool_id, // or pool_id.as_ref() --> same thing
            &[self.pool_config.config_bump],
        ]];

        /// transfer mint_x and mint_y to user's atas
        let amount_x = 0;
        let amount_y = 0; //@audit :: temporary value until fix

        self.transfer_to_user(amount_x, amount_y, signer_seeds);

        /// burn expected lp amounts from user
        self.burn(burn_lp_amount, signer_seeds)?;

        Ok(())
    }

    pub fn transfer_to_user(
        &mut self,
        amount_x: u64,
        amount_y: u64,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let tranfer_x_cpi_accounts = TransferChecked {
            from: self.vault_x.to_account_info(),
            to: self.user_ata_x.to_account_info(),
            mint: self.mint_x.to_account_info(),
            authority: self.pool_config.to_account_info(),
        };

        let tranfer_y_cpi_accounts = TransferChecked {
            from: self.vault_y.to_account_info(),
            to: self.user_ata_y.to_account_info(),
            mint: self.mint_y.to_account_info(),
            authority: self.pool_config.to_account_info(),
        };

        let transfer_x_cpi_context = CpiContext::new(cpi_program.clone(), tranfer_x_cpi_accounts);
        let transfer_y_cpi_context = CpiContext::new(cpi_program.clone(), tranfer_y_cpi_accounts);

        transfer_checked(transfer_x_cpi_context, amount_x, self.mint_x.decimals)?;
        transfer_checked(transfer_y_cpi_context, amount_y, self.mint_y.decimals)?;

        Ok(())
    }

    pub fn burn(&mut self, burn_lp_amount: u64, signer_seeds: &[&[&[u8]]]) -> Result<()> {
        let burn_cpi_accounts = Burn {
            mint: self.mint_lp.to_account_info(),
            from: self.user_ata_lp.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let burn_cpi_context = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            burn_cpi_accounts,
            signer_seeds,
        );

        burn(burn_cpi_context, burn_lp_amount)?;

        Ok(())
    }
}
