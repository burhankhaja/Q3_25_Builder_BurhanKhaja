use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub earned_tokens: u32,
    pub staked_amount: u8,
    pub bump: u8,
}
