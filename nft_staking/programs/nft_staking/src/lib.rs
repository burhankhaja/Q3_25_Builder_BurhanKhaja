pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FSgByULqSBqGPNV4r9cVTdqnrW89RUAKWD4c1UduqQbM");

/*

//! # This is gonna be nft staking program, where a user stakes particular nft collection for certain period of time to claim reward tokens
//!
//! # User will receive X reward tokens per day for staking nft, (rate set by admins),
//!
//! # If user withdraws nft before completing full unix day of staking , they won't receive any rewards for that day



//! =========================================================================================================
//!                              Deffered Considerations :: @later:
//!
//! # if you allow admins to update different reward amounts after deployment, how will you handle this edge case  @audit-issue: imagine bob staked at 3 reward_token /perday for 28 days, on 28th day admin set rewards to 1tokens/perday, bob claims at 29th day , gets 1*29 tokens instead of (3*28) + 1 tokens, how are you going to handle that case then ?? maybe hashmaps of key timestamps to reward rate, then while claiming derive those hashmap timestamps to get relative rewards....Or the easier approach would be to enforce users to claim rewards every week, otherwise rewards won't be claimed, (hence the last_claimed state)... and then admin will only be able to update rewards with one week delay.... +then it would be important to not let users claim rewards if they forgot to claim because how about when admin increases rewards_day, users will get unfair amounts to tokens??? + .....will think about this later ?????????????
//!
//!
//! # : since each nft can have different prices, how do you make sure that reward tokens are minted differently according to the staked value, for that you will need pyth oracle, while mockOracle in the dev testing phase ?
//!
//!
//! #: why not add hashmap data structure to global config to allow staking of various different collections, 1
but do this only after adding oracle configs ?
//!
//!
//! #: Maybe also add user_account closing mechanisms, remember that should thaw all staked nfts and get rewards for user, and if some nft's unfreeze period hasn't reached, forefeit rewards for that particular nft
//! =========================================================================================================



*/

#[program]
pub mod nft_staking {
    use super::*;

    /// Initializes the global configuration for the staking program.
    ///
    /// # Arguments
    /// * `ctx` - Context containing the global config account and authority.
    /// * `reward_tokens_per_day` - Number of reward tokens earned per NFT per day.
    /// * `max_stake` - Maximum number of NFTs a single user can stake.
    /// * `freeze_period` - Cooldown period (in seconds) before NFT can be unstaked.
    ///
    /// # Access
    /// Only callable by an admin authority.

    pub fn initialize_global_config(
        ctx: Context<InitializeGlobalConfig>,
        reward_tokens_per_day: u8,
        max_stake: u8,
        freeze_period: u32,
    ) -> Result<()> {
        ctx.accounts
            .init_global_config(reward_tokens_per_day, max_stake, freeze_period, &ctx.bumps)
    }

    /// Initializes the user state required for staking NFTs.
    ///
    /// # Access
    /// Callable by any wallet once.

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.init_user()
    }

    /// Stakes a whitelisted NFT, beginning reward accrual for the user.
    ///
    /// # Access
    /// Caller must own the NFT and must not exceed max stake count.

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake()
    }

    /// Unstakes a previously staked NFT.
    ///
    /// # Notes
    /// - If unstaked before a full day has passed since last claim/stake, no rewards are granted for that day.
    /// - A freeze period may apply to prevent immediate unstaking.

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }

    /// Claims the accumulated reward tokens based on the staking duration.
    ///
    /// # Notes
    /// - Rewards are calculated based on time elapsed since last claim or stake.
    /// - If reward rates were changed, edge cases must be handled correctly to prevent over/under-rewarding.

    pub fn claim_rewards(ctx: Context<ClaimRewards>) -> Result<()> {
        ctx.accounts.claim_rewards()
    }
}
