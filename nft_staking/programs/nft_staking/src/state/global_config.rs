use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct GlobalConfig {
    pub admin: Pubkey,
    pub reward_tokens_per_day: u8,
    pub max_stake: u8,
    pub freeze_period: u32,
    pub reward_bump: u8,
    pub bump: u8,
}
