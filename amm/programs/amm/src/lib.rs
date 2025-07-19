pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7nenikyBSMTBtdNFaqwTDToRERZyGe3qfoudPfN6dTSX");

#[program]
pub mod amm {
    use super::*;

    // pub fn (ctx: Context<>) -> Result<()> {
    //     Ok()
    // }
    // Lets create the program that creates pools of 2 tokens
    // anyone can create pool , only authority can lock pool

    //@todo:: make some universal fee mechanism , such that expected fees are fetched or some expected range of fee is set, currently set every fee type to 0 , later come back and fix

    ///admin functions
    ///

    //@dev:: @TODO::LATER ::::: how do you make sure only expected address can call this, for this use native rust to bring keypair ///  How do you allow first call to it through only specific address:::::: how do you prevent frontrunning during deployments

    //@natspec::later
    // lock_authority: if user specifies lock_authority then we set it to provided address else we set signer as the authority!!

    pub fn init_authority(ctx: Context<Initialize>, lock_authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(lock_authority, &ctx.bumps)
    }

    /*
    @dev::
    lock::true => locks pool
    lock::false => unlocks pool
     */
    pub fn set_lock(ctx: Context<SetLock>, _pool_id: u16, lock: bool) -> Result<()> {
        ctx.accounts.set_lock(lock)
    }

    // /// User functions
    pub fn create_pool(ctx: Context<CreatePool>, _pool_id: u16, fee: u16) -> Result<()> {
        ctx.accounts.create_pool(_pool_id, fee, &ctx.bumps)
    }

    pub fn deposit_liquidity(ctx: Context<Liquidity>,  _pool_id: u16, mint_lp_amount: u64, max_x: u64, max_y: u64, deadline: i64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw_liquidity(ctx: Context<Liquidity>, _pool_id: u16, burn_lp_amount: u64, max_x: u64, max_y: u64, deadline: i64) -> Result<()> {
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>) -> Result<()> {
        Ok(())
    }
}
