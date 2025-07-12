pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3MZi8MF8xzf7JusYnsUgW3Dia3qNgG1n7jE1Hm19svMi");

#[program]
pub mod escrow {
    // use anchor_lang::prelude::borsh::de;

    use super::*;

    pub fn make_offer(
        ctx: Context<MakeOffer>,
        deposit_amount: u64,
        expect_amount: u64,
        _extra_seed: String,
    ) -> Result<()> {
        ctx.accounts
            .initialize_escrow(&ctx.bumps, deposit_amount, expect_amount)?;
        ctx.accounts.deposit_tokens(deposit_amount)?;
        Ok(())
    }

    pub fn take_offer(ctx: Context<TakeOffer>, _extra_seed: String) -> Result<()> {
        Ok(())
    }

    pub fn cancel_offer(ctx: Context<CancelOffer>, extra_seed: String) -> Result<()> {
        // transfer back offered tokens from vault to maker
        ctx.accounts.withdraw_offered_amounts(&extra_seed)?;
        // close escrow + vault account
        ctx.accounts.close_vault(&extra_seed)?;

        Ok(())
    }

    // Maybe create a helper that gets escrows made by certain users >>>> make sure escrow is dervied from multiple seeds out  of which one must be arbitary string such that users can create multiple offers

    pub fn get_offer_of(
        ctx: Context<GetOffer>,
        _maker_key: Pubkey,
        _extra_seed: String,
    ) -> Result<()> {
        ctx.accounts.get_maker_escrow_data_log()
    }
}

/*

   Make := user makes an offer
   Take := taker takes the offer
   Refund := user claims unsettled offers

*/
