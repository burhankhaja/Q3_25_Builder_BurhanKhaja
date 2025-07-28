use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};
use anchor_spl::token::Mint;

use crate::{Global, Offer};

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,

    #[account(mut)]
    pub seller: SystemAccount<'info>,

    #[account(mut)]
    pub mint: Account<'info, Mint>,

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

    #[account(mut, address = global.treasury)] //@audit :: validate whether this prevents bypass ?
    pub treasury: SystemAccount<'info>, //@audit :: figure out a better way to convert raw Pubkey to type of AccountInfo<'_> // or maybe you can store hashmap type thing which stores address-> T or some raw bytes magic ..... for the later !!!

    // cpi programs
    pub system_program: Program<'info, System>,
}

// close listing account
// we send sol to seller
// we take fee cut from seller profits
// we send nft to buyer

impl<'info> Purchase<'info> {
    pub fn purchase(&mut self) -> Result<()> {
        // pay sol price to seller and cut protocol fee
        self.pay_sol()?;

        // close listing and transfer nft to the buyer
        self.transfer_nft()
    }

    pub fn pay_sol(&mut self) -> Result<()> {
        let system = self.system_program.to_account_info();
        let treasury = self.treasury.to_account_info();
        let buyer = self.buyer.to_account_info();
        let seller = self.seller.to_account_info();
        let price = self.listing.price;

        // Fee calculations
        let now = Clock::get()?.unix_timestamp;
        const TWO_WEEKS: i64 = 2 * 7 * 24 * 60 * 60; // 1,209,600 seconds

        // if 2 weeks have passed after admin had set new_fee, then start charging the new_fee, otherwise keep the old one
        let bips = if ((now - self.global.new_fee_at > TWO_WEEKS) && (self.global.new_fee > 0)) {
            self.global.fee = self.global.new_fee;
            self.global.new_fee = 0;
            self.global.new_fee_at = 0;

            self.global.fee as u64
        } else {
            self.global.fee as u64
        };

        let fee = price.checked_mul(bips).unwrap().checked_div(10000).unwrap(); // price * feebips / 10000
        let price_minus_fee = price - fee;

        // pay nft price to seller
        transfer(
            CpiContext::new(
                system.clone(),
                Transfer {
                    from: buyer.clone(),
                    to: seller,
                },
            ),
            price_minus_fee,
        )?;

        // charge fee on seller's price
        if fee > 0 {
            transfer(
                CpiContext::new(
                    system,
                    Transfer {
                        from: buyer,
                        to: treasury,
                    },
                ),
                fee,
            )?;
        }

        Ok(())
    }

    pub fn transfer_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
