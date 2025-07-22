use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self) -> Result<()> {
        // do some validations, not max_stake, maybe collection validation(or can be done via constraints)

        // freeze user's staked nft
        self.freeze_nft();

        // update stake_account's state
        // update user_account.staked_amount state

        Ok(())
    }

    pub fn freeze_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
