use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct ClaimRewards<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
}

impl<'info> ClaimRewards<'info> {
    pub fn claim_rewards(&mut self) -> Result<()> {
        // Enforcing Checks Effects Interactions Design pattern
        // nullify users_state.earned_tokens & cache
        // update staked_account.last_updated and cache
        // transfer reward tokens to user, while calculating no of staked days, rewards = [(timestamp - staked_at) / 86400 ]* global_config.reward_tokens_per_day
        Ok(())
    }
}
