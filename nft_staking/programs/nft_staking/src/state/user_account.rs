use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    //@store :: points, bump, amount of nts staked,
    pub earned_tokens: u32,
    pub staked_amount: u8,
    pub bump: u8,
}
