use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    pub signer: Signer<'info>,
}
