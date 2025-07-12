use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)] // use the builtin space calculation method instead of implementing on your own
pub struct UserVault {
    pub vault_bump: u8,
}
