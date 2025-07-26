use crate::{error::StakeErrors, GlobalConfig, StakeAccount, UserAccount};
use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, Approve, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,
    pub collection: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_ata: Account<'info, TokenAccount>,

    //@account just used for validation, right here in the context struct
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition",
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    pub edition: Account<'info, MasterEditionAccount>,

    #[account(
        seeds = [b"global_config"], 
        bump = global_config.bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        seeds = [b"user_account", user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,

    ///@audit :: lets say CyberPunk nft, how do you determine which id is being staked, In solana unlike ETH, do different mint address represent ids of same collection.... ??? if yes then add mint.key as seed with some string data

    #[account(
        init,
        payer = user,
        space = StakeAccount::DISCRIMINATOR.len() + StakeAccount::INIT_SPACE,
        seeds = [b"stake_account", user.key().as_ref(), mint.key().as_ref()],
        bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,

    // Cpi Programs
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}

impl<'info> Stake<'info> {
    pub fn stake(&mut self) -> Result<()> {
        // do some validations, not max_stake, maybe collection validation(or can be done via constraints)
        require!(
            self.user_account.staked_amount <= self.global_config.max_stake,
            StakeErrors::MaxStake
        );

        //?! @audit collection validation ??

        // freeze user's staked nft
        self.freeze_nft();

        // update stake_account's state
        self.stake_account.set_inner(StakeAccount {
            owner: *self.user.key,
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: self.stake_account.bump,
        });

        // update user_account.staked_amount state
        self.user_account
            .staked_amount
            .checked_add(1)
            .ok_or(StakeErrors::IntegerOverflow)?;

        Ok(())
    }

    pub fn freeze_nft(&mut self) -> Result<()> {
        let user = self.user.to_account_info();
        let token_program = self.token_program.to_account_info();
        let delegate = self.stake_account.to_account_info();
        let token_account = self.user_ata.to_account_info();
        let metadata_program = self.metadata_program.to_account_info();

        // approve to stake_account as delegate
        let approve_accounts = Approve {
            to: token_account.clone(), //@note to == tokenAccount from where approve will take place
            delegate: delegate.clone(), //@note :: Otherwise you will get "borrowing after move" error in later usages as the ownership is owned already by this
            authority: user.clone(),
        };

        let approve_ctx = CpiContext::new(token_program, approve_accounts);

        approve(approve_ctx, 1)?;

        // let user_key = user.clone().key().as_ref(); //binding to prevent temorary value dropped errors
        let user_key = user.clone().key(); //binding to prevent temorary value dropped errors @note :: as_ref :: causes same binding errors
        let mint_key = self.mint.key(); //for preventing same above errors

        // Freeze nft
        let signer_seeds: &[&[&[u8]]] =
            &[&[b"stake_account", user_key.as_ref(), mint_key.as_ref(), &[self.stake_account.bump]]];

        //@audit_issue :: What kind of access control structure remain after delegating, like :: if original owners retains unfreezing controll even after frozen by delegate, then what is the point of all this ? What if i freeze nft right here without signer seeds, who can unfreeze that ? is it that the one either owner - delegate , whoever freezes gets to unfreeze or what ???? later test this out, for now use signer_seeds  ????

        FreezeDelegatedAccountCpi::new(
            &metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate: &delegate,
                token_account: &token_account,
                edition: &self.edition.to_account_info(),
                mint: &self.mint.to_account_info(),
                token_program: &self.token_program.to_account_info(),
            },
        )
        .invoke_signed(signer_seeds)?;

        Ok(())
    }
}
