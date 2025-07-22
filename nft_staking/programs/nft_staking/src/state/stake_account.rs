use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_at: i64, // why i64 not u64 since time can't be negative ?? --> convention since solana runtime uses i64 for timestamps
    pub bump: u8,
}
