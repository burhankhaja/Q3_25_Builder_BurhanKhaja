use crate::state::Global;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = Global::DISCRIMINATOR.len() + Global::INIT_SPACE,
        seeds = [b"global"],
        bump,

    )]
    pub global: Account<'info, Global>,

    pub system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn initialize_global_account(
        &mut self,
        treasury: Pubkey,
        bumps: &InitializeBumps,
    ) -> Result<()> {
        self.global.set_inner(Global {
            admin: *self.admin.key,
            treasury,
            challenge_ids: 1, // @note: just used to get unique challenge_id for challenge creation
            bump: bumps.global,
        });

        Ok(())
    }
}
