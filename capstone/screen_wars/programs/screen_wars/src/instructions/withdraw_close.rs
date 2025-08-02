use crate::{
    error::Errors,
    state::{Challenge, Global, User},
};

use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(_challenge_id: u32)]
pub struct WithdrawClose<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"global"],
        bump = global.bump,
    )]
    pub global: Account<'info, Global>,

    #[account(
        seeds = [b"challenge", _challenge_id.to_be_bytes().as_ref() ], 
        bump = challenge.bump,
    )]
    pub challenge: Account<'info, Challenge>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref() ], 
        bump = user_account.bump,
        close = user,
    )]
    pub user_account: Account<'info, User>,

    #[account(mut)]
    pub treasury: SystemAccount<'info>, //@audit-issue :: fix later ::

    pub system_program: Program<'info, System>,
}

impl<'info> WithdrawClose<'info> {
    pub fn validate_challenge_has_ended(&mut self) -> Result<()> {
        let now = Clock::get()?.unix_timestamp;
        require!(now > self.challenge.end, Errors::ChallengeNotEnded);
        Ok(())
    }

    pub fn validate_user_is_enrolled_in_challenge(&mut self) -> Result<()> {
        require!(
            self.user_account.challenge_id == self.challenge.challenge_id,
            Errors::NotEnrolled
        );
        Ok(())
    }

    pub fn transfer_sol(&mut self) -> Result<()> {
        let locked_balance = self.user_account.locked_balance;

        if locked_balance > 0 {
            transfer(
                CpiContext::new(
                    self.system_program.to_account_info(),
                    Transfer {
                        from: self.treasury.to_account_info(),
                        to: self.user.to_account_info(),
                    },
                ),
                locked_balance,
            )?;
        }

        Ok(())

        //@audit-issue :: how are you gonna transfer from treasury .... use vault system bro ??? fix ... !!!
        //@dev ::  either use pda system with sub_lamports mech or transfer vault ownership to system and then handover authority to global account ???
    }
}
