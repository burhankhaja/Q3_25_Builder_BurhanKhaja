use crate::{error::ErrorCode, Global, PoolConfig};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(_pool_id: u16)]
pub struct SetLock<'info> {
    #[account(mut)]
    pub lock_authority: Signer<'info>,

    // take
    #[account(
    mut,
    seeds = [b"global"],
    bump = global.bump,

   )]
    pub global: Account<'info, Global>,

    #[account(
     mut,
     seeds = [b"pool_config", _pool_id.to_le_bytes().as_ref()], //@try:: &_pool_id.to_le_bytes() instead
     bump = pool_config.config_bump,
   )]
    pub pool_config: Account<'info, PoolConfig>,
}

impl<'info> SetLock<'info> {
    pub fn set_lock(&mut self, lock: bool) -> Result<()> {
        // validate lock authority
        require!(
            self.lock_authority.key() == self.global.lock_authority,
            ErrorCode::InvalidAuthority
        );

        require!(lock == !self.pool_config.locked, ErrorCode::SameLockState);

       
        self.pool_config.locked = lock;

        Ok(())
    }
}
