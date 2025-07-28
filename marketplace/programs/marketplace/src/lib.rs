pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("6G5KL3C7vYmWdCvkXahjCJmx57VrGDQrync2oR7PgNaw");

/// Marketplace program to enable listing, purchasing, and delisting of NFTs,
/// while enforcing protocol-level admin controls such as fee updates and emergency freezing.
///
/// Admin actions include:
/// - Setting and updating protocol fee (subject to time-based restrictions)
/// - Freezing and unfreezing the protocol
///
/// User actions include:
/// - Listing NFTs for sale
/// - Purchasing listed NFTs
/// - Delisting NFTs (even under protocol freeze, with enforced delay)
///
#[program]
pub mod marketplace {
    use super::*;

    //=====================
    //
    //    admin functions
    //
    //=====================

    /// Initializes the marketplace global state.
    ///
    /// @param ctx Accounts context including global state and payer.
    /// @param treasury Public key of the treasury account to collect fees.
    /// @param fee Initial protocol fee in basis points (1% = 100 bps).
    ///
    /// @notice Can only be called once by the admin to initialize protocol configuration.
    pub fn initialize(ctx: Context<Initialize>, treasury: Pubkey, fee: u16) -> Result<()> {
        ctx.accounts.initialize(treasury, fee, &ctx.bumps)
    }

    /// Freezes or unfreezes the protocol.
    ///
    /// @param ctx Accounts context including admin and global config.
    /// @param freeze Boolean flag to indicate freeze (true) or unfreeze (false).
    ///
    /// @notice While frozen:
    /// - Listing and purchasing NFTs is disabled.
    /// - Delisting is still allowed, but delayed by 1 week to prevent sudden escape.
    pub fn freeze_thaw(ctx: Context<FreezeThaw>, freeze: bool) -> Result<()> {
        ctx.accounts.set(freeze)
    }

    /// Updates the protocol fee, with strict time-based rules:
    ///
    /// @param ctx Accounts context including admin and global config.
    /// @param new_fee New fee value in basis points upto 0.5% limit
    ///
    /// @notice Admin can only update the fee if:
    /// - At least 1 week has passed since the last update.
    /// - The new fee takes effect **2 weeks after** this transaction for sellers.
    ///
    /// @dev Prevents spammy or malicious fee changes. Encourages predictability.
    pub fn update_fee(ctx: Context<UpdateFee>, new_fee: u16) -> Result<()> {
        ctx.accounts.update(new_fee)
    }

    //=====================
    //
    //    User functions
    //
    //=====================

    /// List an NFT for sale on the marketplace.
    ///
    /// @param ctx Accounts context including seller, NFT metadata, and offer PDA.
    /// @param price Listing price in native sol lamports.
    ///
    /// @notice Listing is disallowed while protocol is frozen.
    /// @dev Creates a new `Offer` PDA and stores price + metadata.
    pub fn list_nft(ctx: Context<List>, price: u64) -> Result<()> {
        ctx.accounts.list(price, &ctx.bumps)
    }

    /// Delist an NFT that was previously listed.
    ///
    /// @param ctx Accounts context including seller and offer.
    ///
    /// @notice Delisting is delayed by 1 week, if protocol is currently frozen.
    /// @dev Prevents users from instantly withdrawing NFTs during emergency freezes.
    pub fn delist_nft(ctx: Context<Delist>) -> Result<()> {
        ctx.accounts.delist(&ctx.bumps)
    }

    /// Purchase a listed NFT from the marketplace.
    ///
    /// @param ctx Accounts context including buyer, seller, offer, and vaults.
    ///
    /// @notice Purchasing is disallowed while protocol is frozen.
    /// @dev Handles SOL transfer, fee distribution, and NFT ownership change.
    pub fn purchase_nft(ctx: Context<Purchase>) -> Result<()> {
        ctx.accounts.purchase(&ctx.bumps)
    }
}
