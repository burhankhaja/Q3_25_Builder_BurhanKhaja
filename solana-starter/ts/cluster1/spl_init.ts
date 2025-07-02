import { Keypair, Connection, Commitment, PublicKey } from "@solana/web3.js";
import { createMint } from '@solana/spl-token';
import wallet from "../../Turbin3-wallet.json"

// Import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

/**
 function createMint(connection: Connection, payer: Signer, mintAuthority: PublicKey, freezeAuthority: PublicKey | null, decimals: number, keypair?: Keypair, confirmOptions?: ConfirmOptions, programId?: PublicKey):
 */


(async () => {
    try {
        // Start here
        // const mint = ???

        // done :: https://explorer.solana.com/tx/6VveWch98R1RiGqjfkwXMgvUPUBaZtET29YM6zvNjry5C4QhsksrNYSD24wdbcB6Y8dV8BNmLN2aDLB7CovkAUQ?cluster=devnet


        // Token:: https://explorer.solana.com/address/2zszKQQNpTtGAv419BRdtSLgbEavUJFZRXxPCffh935g?cluster=devnet

        // after change metadata for it, right now it is named as Unknown token

        const kp = Keypair.generate();

        const mint = await createMint(connection, keypair, keypair.publicKey, keypair.publicKey,6, kp,  { commitment: "confirmed" }, new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()
