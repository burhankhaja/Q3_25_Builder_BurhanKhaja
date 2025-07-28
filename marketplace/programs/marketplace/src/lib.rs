pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6G5KL3C7vYmWdCvkXahjCJmx57VrGDQrync2oR7PgNaw");

#[program]
pub mod marketplace {

    use super::*;

    //@audit-laters :: handle precision loss ::: considers 1% of 250, since u* stores 2 while real maths is 2.5, remember not to use floating points for onchain programs, consider solidity precision math style >>> maybe

    // pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    //     initialize::handler(ctx)
    // }

    // pub fn (ctx:Context<>) -> Result<()> {
    //     Ok(())
    // }

    //=====================
    //
    //    admin functions
    //
    //=====================
    pub fn initialize(ctx: Context<Initialize>, treasury: Pubkey, fee: u16) -> Result<()> {
        ctx.accounts.initialize(treasury, fee, &ctx.bumps)
    }

    pub fn freeze_thaw(ctx: Context<FreezeThaw>, freeze: bool) -> Result<()> {
        Ok(())
    }

    pub fn update_fee(ctx: Context<UpdateFee>, new_fee: u16) -> Result<()> {
        ctx.accounts.update(new_fee)
    }

    //=====================
    //
    //    User functions
    //
    //=====================

    pub fn list_nft(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.list(price, &ctx.bumps)
    }

    pub fn delist_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist()
    }

    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase()
    }

    //
}
