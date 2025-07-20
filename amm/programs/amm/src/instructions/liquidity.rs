use crate::error::ErrorCode;
use crate::PoolConfig;
use anchor_lang::prelude::*;
use anchor_spl::associated_token::{create, get_associated_token_address};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        burn, mint_to, transfer_checked, Burn, Mint, MintTo, Token, TokenAccount, TransferChecked,
    },
}; //@audit :: merge later in above statement

use constant_product_curve::ConstantProduct;
use integer_sqrt::IntegerSquareRoot;

#[derive(Accounts)]
#[instruction(_pool_id: u16)]
pub struct Liquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    /// deposit tokens
    #[account(mint::token_program = token_program)]
    pub mint_x: Account<'info, Mint>,

    #[account(mint::token_program = token_program)]
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

    // /// Optional ATA for mint_lp owned by system_program (used to lock minimum_liquidity on bootstrap)
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint_lp,
        associated_token::authority = system_program,
        associated_token::token_program = token_program
    )]
    pub locked_liquidity_ata: Option<Account<'info, TokenAccount>>,

    /// Cpi programs
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Liquidity<'info> {
    const MIN_LOCKED_LIQUIDITY: u64 = 1_000; //@audit :: should i scale this with mint_lp.decimals , what if some mint_lp has 24 decimals ?
    pub fn deposit(
        &mut self,
        _pool_id: u16,
        mint_lp_amount: u64,
        max_x: u64,
        max_y: u64,
        deadline: i64,
    ) -> Result<()> {
        require!(
            Clock::get()?.unix_timestamp <= deadline,
            ErrorCode::ExpiredTx
        );
        require!(!self.pool_config.locked, ErrorCode::LockedPoolId);
        require!(mint_lp_amount > 0, ErrorCode::InvalidAmount);

        let (amount_x, amount_y, lp_to_mint, is_first_deposit) = match self.mint_lp.supply == 0 {
            ////  Case 1: First LP depositor (bootstrap)
            true => {
                // For initial mint, match exact max_x and max_y
                let sqrt_k = (max_x as u128)
                    .checked_mul(max_y as u128)
                    .unwrap()
                    .integer_sqrt();

                // let minimum_liquidity = 1_000;
                let lp_tokens = sqrt_k
                    .checked_sub(Self::MIN_LOCKED_LIQUIDITY as u128)
                    .unwrap(); //do error handling later

                // The goal is to lock the minimum liquidity in an account that no one can access or use, i.e,  effectively burning it!

                //Psuedo_code :::  self.lock_minimum_liquidity(minimum_liquidity)?;

                (max_x, max_y, lp_tokens, true)
            }
            //// Case 2: Normal LP deposit
            false => {
                let amounts = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.mint_lp.supply,
                    mint_lp_amount,
                    1_000_000, // since mintlp has 6 decimals
                )
                .unwrap(); //do error handling later

                require!(
                    amounts.x <= max_x && amounts.y <= max_y,
                    ErrorCode::BrokenSlippage
                );
                (amounts.x, amounts.y, mint_lp_amount as u128, false)
            }
        };

        //// get deposit tokens from the user
        self.transfer_from_user(amount_x, amount_y)?;

        //// mint lp tokens for the user
        self.mint(_pool_id, lp_to_mint as u64, is_first_deposit)?;

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

    pub fn mint(
        &mut self,
        _pool_id: u16,
        mint_lp_amount: u64,
        is_first_deposit: bool,
    ) -> Result<()> {
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

        if is_first_deposit {
            self.lock_minimum_liquidity(_pool_id, signer_seeds)?;
        }

        mint_to(mint_cpi_context, mint_lp_amount)?;

        Ok(())
    }

    pub fn lock_minimum_liquidity(
        &mut self,
        _pool_id: u16,
        signer_seeds: &[&[&[u8]]],
    ) -> Result<()> {
        let pool_id = _pool_id.to_le_bytes();

        let locked_account = self.locked_liquidity_ata.as_ref().unwrap(); //@later add error handling for Option.None case!

        let cpi_ctx = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            MintTo {
                mint: self.mint_lp.to_account_info(),
                to: locked_account.to_account_info(),
                authority: self.pool_config.to_account_info(),
            },
            signer_seeds,
        );

        mint_to(cpi_ctx, Self::MIN_LOCKED_LIQUIDITY)?; //@notice use of `Self`` with capital S instead of self for accessing the constants defined inside impl
        Ok(())
    }

    ///// withdraw user
    ///
    ///
    ///
    pub fn withdraw(
        &mut self,
        _pool_id: u16,
        burn_lp_amount: u64,
        min_x: u64,
        min_y: u64,
        deadline: i64,
    ) -> Result<()> {
        // validate key states, lock, amounts, slippage, expiration,  first deposit vs normal deposit cases
        require!(
            Clock::get()?.unix_timestamp <= deadline,
            ErrorCode::ExpiredTx
        );
        require!(!self.pool_config.locked, ErrorCode::LockedPoolId);
        require!(burn_lp_amount > 0, ErrorCode::InvalidAmount);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.mint_lp.supply,
            burn_lp_amount,
            1_000_000, // since mint_lp has 6 decimals
        )
        .unwrap();

        let amount_x = amounts.x;
        let amount_y = amounts.y;

        // validate slippage
        require!(
            amount_x >= min_x && amount_y >= min_y,
            ErrorCode::BrokenSlippage
        );

        let pool_id = _pool_id.to_be_bytes(); // to prevent "data moved" errors

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool_config",
            &pool_id, // or pool_id.as_ref() --> same thing
            &[self.pool_config.config_bump],
        ]];

        //// burn users lp amounts in accordance to the transfer to deposit tokens to him
        self.transfer_to_user(amount_x, amount_y, signer_seeds)?;
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

        let transfer_x_cpi_context =
            CpiContext::new_with_signer(cpi_program.clone(), tranfer_x_cpi_accounts, signer_seeds);
        let transfer_y_cpi_context =
            CpiContext::new_with_signer(cpi_program.clone(), tranfer_y_cpi_accounts, signer_seeds);

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
