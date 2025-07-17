pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FSgByULqSBqGPNV4r9cVTdqnrW89RUAKWD4c1UduqQbM");

// first commit ::
/*
Add state and only basic logic of state initialization , nft freezing/thawing and lil bit of tests
 */

#[program]
pub mod nft_staking {
    use super::*;

    pub fn initialize_global_config(
        ctx: Context<InitializeGlobalConfig>,
        points_per_stake: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_global_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        Ok(())
    }

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
}
