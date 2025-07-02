import { Connection, Keypair, PublicKey, SystemProgram  } from "@solana/web3.js"
import { Program, Wallet, AnchorProvider } from "@coral-xyz/anchor"
import IDL from "./programs/Turbin3_prereq.json";
import type { Idl } from "@coral-xyz/anchor";

import wallet from "./Turbin3-wallet.json"
const MPL_CORE_PROGRAM_ID = new
    PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d");
const SYSTEM_PROGRAM_ID = SystemProgram.programId;    


// setting up keypair and connection
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const connection = new Connection("https://api.devnet.solana.com");
const provider = new AnchorProvider(connection, new Wallet(keypair), {
    commitment: "confirmed"
});

// program setup
const program = new Program(IDL as Idl, provider);

// derive PDA
// Create the PDA for our enrollment account
const account_seeds = [
    Buffer.from("prereqs"),
    keypair.publicKey.toBuffer(),
];
const [account_key, _account_bump] = PublicKey.findProgramAddressSync(account_seeds, program.programId);

// debug
// console.log("Keypair: ", keypair.publicKey.toBase58());
// console.log("PDA: ", account_key.toBase58());



//@note during test := commeted out for the second tx to pass `submitTs()`
// // Execute the initialize transaction
(async () => {
    try {
        const txhash = await program.methods
            .initialize("burhankhaja")
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair])
            .rpc();
        console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();

// setup for submitTs
const mintCollection = new PublicKey("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2");
const mintTs = Keypair.generate();

//@note :: additional :: just in case :: keeping track of mintTs account
// Keeping track of mintTs : might need it
console.log(` Save mintTs Now ::: [${mintTs.secretKey}]`);
const [collection_pda, _collection_bump] = PublicKey.findProgramAddressSync([Buffer.from("collection"), mintCollection.toBuffer()], program.programId);


// debug
// console.log(`collection_pda: ${collection_pda.toBase58()}`); // 5xstXUdRJKxRrqbJuo5SAfKf68y7afoYwTeH1FXbsA3k
// console.log("system program: ", SystemProgram.programId.toBase58());
// console.log("Keypair: ", keypair.publicKey.toBase58());


// Execute the submitTs transaction
(async () => {
    try {
        const txhash = await program.methods
            .submitTs()
            .accountsPartial({
                user: keypair.publicKey,
                account: account_key,
                mint: mintTs.publicKey,
                collection: mintCollection,
                authority: collection_pda,
                mpl_core_program: MPL_CORE_PROGRAM_ID,
                system_program: SYSTEM_PROGRAM_ID,
            })
            .signers([keypair, mintTs])
            .rpc();
        console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
    } catch (e) {
        console.error(`Oops, something went wrong: ${e}`);
    }
})();