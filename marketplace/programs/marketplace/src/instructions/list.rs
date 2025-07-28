use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
    token::{transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{error::MarketplaceErrors, Global, Offer};

#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::authority = seller,
        associated_token::mint = mint,
    )]
    pub seller_ata: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = seller,
        seeds = [b"vault", mint.key().as_ref()],
        bump,
        token::authority = vault,
        token::mint = mint,
    )]
    pub vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = seller,
        space = 8 + Offer::INIT_SPACE,
        seeds = [b"listing", seller.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub listing: Account<'info, Offer>,

    #[account(
        seeds = [b"global"],
        bump = global.bump
    )]
    pub global: Account<'info, Global>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
            ],
            seeds::program = metadata_program.key(),
            bump
        )]
    metadata: Account<'info, MetadataAccount>,

    // cpi programs
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> List<'info> {
    pub fn list(&mut self, price: u64, bumps: &ListBumps) -> Result<()> {
        require!(!self.global.frozen, MarketplaceErrors::ProtocolFrozen);

        // transfer nft to the offer account
        self.deposit_nft()?;

        // create and initialize offer account of user
        self.listing.set_inner(Offer {
            seller: (*self.seller.key),
            price: (price),
            bump: (bumps.listing),
        });

        Ok(())
    }

    pub fn deposit_nft(&mut self) -> Result<()> {
        //@later :: validate nft collection

        // deposit nft
        let token_program = self.token_program.to_account_info();
        transfer(
            CpiContext::new(
                token_program,
                Transfer {
                    from: self.seller_ata.to_account_info(),
                    to: self.vault.to_account_info(),
                    authority: self.seller.to_account_info(),
                },
            ),
            1,
        )?;

        Ok(())
    }
}
