use crate::Global;
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
    pub fn init(&mut self, lock_authority: Option<Pubkey>, bumps: &InitializeBumps) -> Result<()> {
        let authority = match lock_authority {
            Some(authority_address) => authority_address,

            None => self.admin.key(),
        };

        self.global.set_inner(Global {
            lock_authority: authority,
            bump: bumps.global,
        });

        Ok(())
    }
}
