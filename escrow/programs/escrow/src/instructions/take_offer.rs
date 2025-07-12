use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct TakeOffer<'info> {
    pub taker: Signer<'info>,

    // taker_maker_offered_token_ata // Oh shit naming convention backfired in case of taker, since one parties offered is another ones expected

    // maker_offered_token....

    // maker escrow

    // maker vault

    // maker expected mint
    // maker expected mint ata

    // ata_program , system_program, token_program
}
