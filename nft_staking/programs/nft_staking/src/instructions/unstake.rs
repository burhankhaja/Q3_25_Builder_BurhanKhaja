use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Unstake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> Unstake<'info> {
    pub fn unstake(&mut self) -> Result<()> {
        // validate stake + freeze_period

        // freeze user's staked nft
        self.unfreeze_nft();

        // close user's stake_account, via constraints
        // update user_account.staked_amount state, (decrease by 1)

        Ok(())
    }

    pub fn unfreeze_nft(&mut self) -> Result<()> {
        Ok(())
    }
}
