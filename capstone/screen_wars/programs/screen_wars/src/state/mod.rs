use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Global {
    pub admin: Pubkey,
    pub treasury: Pubkey,
    pub treasury_profits: u64,
    pub challenge_ids: u32, // since counter, maybe use explicit naming
    pub challenge_creation_paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Challenge {
    pub creator: Pubkey,
    pub challenge_id: u32,
    pub daily_timer: i64,
    pub start: i64,
    pub end: i64,
    pub total_slashed: u64,
    pub winner: Pubkey,
    pub winner_streak: u8,
    pub winner_has_claimed: bool,
    pub creator_has_claimed: bool,
    pub total_participants: u32,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct User {
    pub user: Pubkey,
    pub challenge_id: u32,
    pub locked_balance: u64,
    pub streak: u8,
    pub bump: u8,
}
