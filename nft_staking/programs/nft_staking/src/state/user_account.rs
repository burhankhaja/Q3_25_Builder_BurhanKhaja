use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    //@store :: points, bump, amount of nts staked,
    pub points: u32,
    pub staked_amount: u8,
    pub bump: u8,
}
