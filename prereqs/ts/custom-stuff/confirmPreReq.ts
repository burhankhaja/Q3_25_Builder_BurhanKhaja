import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program,  } from "@coral-xyz/anchor";
import IDL from "../programs/Turbin3_prereq.json";
import type { Idl } from "@coral-xyz/anchor";

const myTurbinWallet = new PublicKey("5gLcyi3EZK56aVc6XEurDiXvk74s9xMHKhFAsAAyZfLg"); 
const programId = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");

(async () => {
  const connection = new Connection("https://api.devnet.solana.com");
  const provider = new AnchorProvider(connection, {} as any, {
    commitment: "confirmed",
  });

  const program = new Program(IDL as Idl, provider);


  // Derive PDA: [ "prereqs", user ]
  const [pda] = PublicKey.findProgramAddressSync(
    [Buffer.from("prereqs"), myTurbinWallet.toBuffer()],
    programId
  );

  console.log("PDA:", pda.toBase58());

  try {
const account = await (program.account as any)["applicationAccount"].fetch(pda);
    console.log("Fetched Data:", account);
  } catch (err) {
    console.error("Fetch failed:", err);
  }
})();

// // quick test
// (async() => {
// const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")
// const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');
// const mint = new PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");
// const owner = new PublicKey("45ruCyfdRkWpRNGEqWzjCiXRHkZs8WXCLQ67Pnpye7Hp");

//  const [ata] = PublicKey.findProgramAddressSync(
//         [owner.toBuffer(), TOKEN_PROGRAM_ID.toBuffer(), mint.toBuffer()],
//         ASSOCIATED_TOKEN_PROGRAM_ID,
//     );

//     console.log(`ATA ACCOUNT : ${ata}`)

// })()