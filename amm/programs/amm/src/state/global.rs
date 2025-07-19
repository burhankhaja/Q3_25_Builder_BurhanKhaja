use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Global {
    pub lock_authority: Pubkey,
    pub bump: u8,
}
