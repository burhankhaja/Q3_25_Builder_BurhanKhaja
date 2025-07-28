use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::MarketplaceErrors, Global, Offer};

#[derive(Accounts)]
pub struct Delist<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = seller,
        associated_token::mint = mint,
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        token::authority = vault,
        token::mint = mint,
        close = seller,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"listing", seller.key().as_ref(), mint.key().as_ref()],
        bump = listing.bump,
        close = seller,
    )]
    pub listing: Account<'info, Offer>,

    #[account(
        seeds = [b"global"],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    // cpi programs
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Delist<'info> {
    pub fn delist(&mut self, bumps: &DelistBumps) -> Result<()> {
        // Only delisting is allowed when the protocol is in frozen state but with 1 week delay
        if self.global.frozen {
            let now = Clock::get()?.unix_timestamp;
            const ONE_WEEK: i64 = 7 * 24 * 60 * 60; // 604,800 seconds
            require!(
                now - self.global.frozen_at >= ONE_WEEK,
                MarketplaceErrors::FrozenDelistDelay
            );
        }
        // close listing and withdraw nft back to seller
        self.withdraw_nft(bumps)
    }

    pub fn withdraw_nft(&mut self, bumps: &DelistBumps) -> Result<()> {
        transfer(
            CpiContext::new_with_signer(
                self.token_program.to_account_info(),
                Transfer {
                    from: self.vault.to_account_info(),
                    to: self.seller_ata.to_account_info(),
                    authority: self.vault.to_account_info(),
                },
                &[&[b"vault", self.mint.key().as_ref(), &[bumps.vault]]],
            ),
            1,
        )
    }
}
