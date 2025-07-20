pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("7nenikyBSMTBtdNFaqwTDToRERZyGe3qfoudPfN6dTSX");

/// Automated Market Maker (AMM) program for liquidity pools, swaps, and pool management.
///
/// This program allows users to create liquidity pools, deposit and withdraw liquidity,
/// perform token swaps, and control pool locking state to prevent actions during maintenance or emergencies.
///
/// # Instructions
/// - `init_authority`: Initialize or change pool lock authority.
/// - `set_lock`: Lock or unlock a pool to restrict deposits/withdrawals/swaps.
/// - `create_pool`: Create a new liquidity pool with specified fee parameters.
/// - `deposit_liquidity`: Add liquidity tokens to a pool, minting LP tokens.
/// - `withdraw_liquidity`: Burn LP tokens to withdraw underlying tokens from pool.
/// - `swap`: Swap tokens in a pool following constant product formula.
///
/// # Note
/// - Many instructions include deadline parameter to protect against front-running and stale transactions.
/// - Pools can be locked/unlocked by authorized accounts to prevent interaction during upgrades.
///
/// # Program ID
/// `7nenikyBSMTBtdNFaqwTDToRERZyGe3qfoudPfN6dTSX`
///
/// # @todo
/// - Implement state tracking on vault balance to prevent inflation attacks
/// - Add frontrunning protection in `init_authority` by allowing calls only from a predefined Pubkeys`.
/// - Implement fee logic in swap and liquidity instructions (currently ignored).

#[program]
pub mod amm {
    use super::*;

    //////////////////////////////
    ///  Admin Functions
    //////////////////////////////

    /// Initialize or set the lock authority of the program.
    ///
    /// # Parameters
    /// - `ctx`: Context containing accounts required for initialization.
    /// - `lock_authority`: Optional new lock authority public key.
    ///
    /// # Behavior
    /// If `lock_authority` is provided, sets it as the lock authority; otherwise, sets the signer as authority.
    /// This authority can lock or unlock pools.
    pub fn init_authority(ctx: Context<Initialize>, lock_authority: Option<Pubkey>) -> Result<()> {
        ctx.accounts.init(lock_authority, &ctx.bumps)
    }

    /// Lock or unlock a liquidity pool by pool ID.
    ///
    /// # Parameters
    /// - `ctx`: Context containing accounts for pool config.
    /// - `_pool_id`: Identifier of the pool to lock/unlock.
    /// - `lock`: `true` to lock the pool, `false` to unlock.
    ///
    /// # Behavior
    /// Prevents deposits, withdrawals, and swaps when locked.
    pub fn set_lock(ctx: Context<SetLock>, _pool_id: u16, lock: bool) -> Result<()> {
        ctx.accounts.set_lock(lock)
    }

    /// Create a new liquidity pool with specified fee.
    ///
    /// # Parameters
    /// - `ctx`: Context with accounts needed to create pool.
    /// - `_pool_id`: Unique identifier for the new pool.
    /// - `fee`: Fee rate in basis points applied to swaps in this pool.
    ///
    /// # Behavior
    /// Initializes pool config, token vaults, and LP mint.
    pub fn create_pool(ctx: Context<CreatePool>, _pool_id: u16, fee: u16) -> Result<()> {
        ctx.accounts.create_pool(_pool_id, fee, &ctx.bumps)
    }

    //////////////////////////////
    /// User Functions
    //////////////////////////////

    /// Deposit tokens into a liquidity pool to mint LP tokens.
    ///
    /// # Parameters
    /// - `ctx`: Context with user and pool accounts.
    /// - `_pool_id`: Pool identifier to deposit into.
    /// - `mint_lp_amount`: Amount of LP tokens user expects to mint.
    /// - `max_x`: Max amount of token X user is willing to deposit.
    /// - `max_y`: Max amount of token Y user is willing to deposit.
    /// - `deadline`: Unix timestamp after which this transaction will fail.
    ///
    /// # Behavior
    /// Transfers tokens from user to vaults and mints LP tokens to user.
    /// Enforces slippage and deadline.
    pub fn deposit_liquidity(
        ctx: Context<Liquidity>,
        _pool_id: u16,
        mint_lp_amount: u64,
        max_x: u64,
        max_y: u64,
        deadline: i64,
    ) -> Result<()> {
        ctx.accounts
            .deposit(_pool_id, mint_lp_amount, max_x, max_y, deadline)
    }

    /// Withdraw tokens from a liquidity pool by burning LP tokens.
    ///
    /// # Parameters
    /// - `ctx`: Context with user and pool accounts.
    /// - `_pool_id`: Pool identifier to withdraw from.
    /// - `burn_lp_amount`: Amount of LP tokens to burn.
    /// - `min_x`: Minimum acceptable amount of token X to receive.
    /// - `min_y`: Minimum acceptable amount of token Y to receive.
    /// - `deadline`: Unix timestamp after which this transaction will fail.
    ///
    /// # Behavior
    /// Burns LP tokens and transfers underlying tokens to user.
    /// Enforces slippage and deadline.
    pub fn withdraw_liquidity(
        ctx: Context<Liquidity>,
        _pool_id: u16,
        burn_lp_amount: u64,
        min_x: u64,
        min_y: u64,
        deadline: i64,
    ) -> Result<()> {
        ctx.accounts
            .withdraw(_pool_id, burn_lp_amount, min_x, min_y, deadline)
    }

    /// Swap tokens inside a liquidity pool.
    ///
    /// # Parameters
    /// - `ctx`: Context with user and pool accounts.
    /// - `_pool_id`: Pool identifier for the swap.
    /// - `is_x`: `true` if swapping token X for Y, `false` for Y to X.
    /// - `amount_in`: Amount of input token user sends.
    /// - `min_out`: Minimum acceptable output token amount.
    /// - `deadline`: Unix timestamp after which this transaction will fail.
    ///
    /// # Behavior
    /// Executes a constant product swap, enforcing slippage and deadline.
    pub fn swap(
        ctx: Context<Swap>,
        _pool_id: u16,
        is_x: bool,
        amount_in: u64,
        min_out: u64,
        deadline: i64,
    ) -> Result<()> {
        ctx.accounts
            .swap(_pool_id, is_x, amount_in, min_out, deadline)
    }
}
