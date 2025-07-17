use anchor_lang::prelude::*;

//@store ::  store the owner, mint (nft) that this account represents, time when this account was updated, denoting the stake period, and ofcourse the bump
#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub last_updated: i64, // why i64 not u64 since time can't be negative ?? --> convention since solana runtime uses i64 for timestamps
    pub bump: u8,
}
