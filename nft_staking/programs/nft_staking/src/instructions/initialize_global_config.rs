use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::GlobalConfig;

//@limitation:: we are not validating admin key against certain keypair , wouldn't anybody frontrun and annoy deployment processes

#[derive(Accounts)]
pub struct InitializeGlobalConfig<'info> {
    //@dev:: first of all we need to create global config account and link that up with reward token that needs to be minted for rewarding purposes
    #[account(mut)]
    pub admin: Signer<'info>, //@audit:: validate against certain keypairs to avoid greifing attack !!! use stored dumming keypair file and push that to repo <expected_no_gitIgnore_dummy_keypair_for_admin_verification.json>  or during that certain audit on cantina , there were some crazy checks during runtime .... implement that

    #[account(
        init,
        payer = admin,
        space =  GlobalConfig::DISCRIMINATOR.len() +  GlobalConfig::INIT_SPACE,
        seeds = [b"global_config"], //@audit :: check what diff adding .as_ref makes despite doing b" // its byte array vs slice thing, but still check out any diffs while testing with ts /// findProgramSync????? with both seed types 
        bump,
    )]
    pub global_config: Account<'info, GlobalConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards"], // any diff ::::: same above question::::: b"".as_ref with global_config.key().as_ref .. vs .. b""" , global_config.key().as_ref .... same pda derivation??? how would slice array differ while in cpi contexts those &[&[&[...]]] ///// test this behaviour out
        bump,
        mint::decimals = 9, // lets use 9 decimal precision , same as sol
        mint::authority = global_config,

    )]
    pub reward_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>, //@audit :: remember Interface<'info, TokenInterface> || check pro/cons
}

impl<'info> InitializeGlobalConfig<'info> {
    pub fn init_global_config(
        &mut self,
        reward_tokens_per_day: u8,
        max_stake: u8,
        freeze_period: u32,
        bumps: &InitializeGlobalConfigBumps,
    ) -> Result<()> {
        self.global_config.set_inner(GlobalConfig {
            admin: *self.admin.key, //@note :: instead of dereference you can directly use self.admin.key() // with parenthesis
            reward_tokens_per_day: reward_tokens_per_day,
            max_stake: max_stake,
            freeze_period: freeze_period,
            reward_bump: bumps.reward_mint,
            bump: bumps.global_config,
        });

        Ok(())
    }
}
