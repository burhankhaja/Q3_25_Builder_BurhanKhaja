use anchor_lang::prelude::*;

use crate::state::Escrow;

#[derive(Accounts)]
#[instruction(maker_key: Pubkey, extra_seed: String,)]
pub struct GetOffer<'info> {
    #[account(
        mut,
        seeds = [b"escrow", maker_key.as_ref(), extra_seed.as_ref()],
        bump = maker_escrow.bump,

    )]
    maker_escrow: Account<'info, Escrow>,
}

impl<'info> GetOffer<'info> {
    pub fn get_maker_escrow_data_log(&mut self) -> Result<()> {
        msg!("{:?}", self.maker_escrow); // in the test check how this shows the data, user gets what data represents ? fine: refactor_code || use individual logging on self.maker_escrow.data_value[0]....n

        Ok(())
    }
}
