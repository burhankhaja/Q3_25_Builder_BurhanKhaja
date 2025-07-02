import { Connection, PublicKey } from "@solana/web3.js";
import { AnchorProvider, Program } from "@coral-xyz/anchor";
import IDL from "../programs/Turbin3_prereq.json";
import type { Idl } from "@coral-xyz/anchor";

const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const programId = new PublicKey("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM");

(async () => {
  const connection = new Connection("https://api.devnet.solana.com");
  const provider = new AnchorProvider(connection, {} as any, {
    commitment: "confirmed",
  });

  const program = new Program(IDL as Idl, provider);

  // Derive PDA for collection: seeds = ["collection", mintCollection]
  const [collectionPda, bump] = PublicKey.findProgramAddressSync(
    [Buffer.from("collection"), mintCollection.toBuffer()],
    program.programId
  );

  console.log("Collection PDA:", collectionPda.toBase58()); //5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k // find if this was provided by other participants
  console.log("Collection PDA bump:", bump);
})();
