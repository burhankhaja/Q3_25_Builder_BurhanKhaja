import * as anchor from "@coral-xyz/anchor";
import { Escrow } from "../target/types/escrow";
import {Keypair} from "@solana/web3.js";
import { TOKEN_PROGRAM_ID , getOrCreateAssociatedTokenAccount, getAccount  } from "@solana/spl-token";


describe("escrow", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.escrow as anchor.Program<Escrow>;

  


//   it("Make offer!", async () => {
//       let deposit_amount = 99;
//       let expect_amount;
//       let extra_seed;

//       const dummy_offered_mint = Keypair.generate().publicKey;

// //       const makerPublicKey = provider.wallet.publicKey; // your maker signer
// // const mintPublicKey = dummy_offered_mint;

// // // Derive ATA address for maker's offered mint
// // const makerOfferedAta = await getOrCreateAssociatedTokenAccount(
// //   provider.connection,
// //   provider.wallet.payer,      // payer Keypair or Signer
// //   mintPublicKey,              // mint
// //   provider.wallet.publicKey,             // owner of the ATA
// // );

//       // const beforeBalance = await getAccount(provider.connection, makerOfferedAta.address);


//       const tx = await program.methods.makeOffer(deposit_amount, expect_amount, extra_seed).accounts({
//         signer: provider.wallet.publicKey,
//         offeredMint: dummy_offered_mint,
//         expectedMint: Keypair.generate().publicKey,
//         // makerOfferedAta, //auto-derive
//         // escrow, //auto
//         // vault, // auto
//         tokenProgram: TOKEN_PROGRAM_ID,



//       });

// // const afterBalance = await getAccount(provider.connection, makerOfferedAta.address);

// // console.log(`before: ${beforeBalance}`)
// // console.log(`after: ${afterBalance}`)

// // try fetching account instead of why tx is passing ??


//   });    


  /**  #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mint::token_program = token_program
    )]
    pub offered_mint: InterfaceAccount<'info, Mint>,

    #[account(
        mint::token_program = token_program
    )]
    pub expected_mint: InterfaceAccount<'info, Mint>,

    // since i have to take tokens from user , pass his ATA
    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = signer,
        associated_token::token_program = token_program,
    )]
    pub maker_offered_ata: InterfaceAccount<'info, TokenAccount>,

    // create an escrow
    #[account(
        init,
        payer = signer,
        space = 8 + Escrow::INIT_SPACE,
        seeds = [b"escrow", signer.key().as_ref(), extra_seed.as_ref()],
        bump,

    )]
    pub escrow: Account<'info, Escrow>,

    // create an ata for escrow
    #[account(
        mut,
        associated_token::mint = offered_mint,
        associated_token::authority = escrow,
        associated_token::token_program = token_program,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_progra */

   


  // @TEST_LATER : remove account discriminator space while Escrow creation, and figure out how does creation and fetching of that account causes deserialization problems , since [[space = 8 + Escrow::INIT_SPACE ]] try {space = Escrow::INIT_SPACE}

});
