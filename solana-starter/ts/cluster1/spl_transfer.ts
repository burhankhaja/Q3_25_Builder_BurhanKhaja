import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../../Turbin3-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// keypair && connection
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

// Mint address
const mint = new PublicKey("2zszKQQNpTtGAv419BRdtSLgbEavUJFZRXxPCffh935g");

// Recipient address
const alice = new PublicKey("CYdkpczCP4kwhRPVC2A2edowtkshbdTFPdwaFrZ3DJzz");
const token_decimals = 1_000_000n;

(async () => {
    try {
        // From && To ATAs
        const from_ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey)
        const to_ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, alice)
        // amount
        const amount = 10n * token_decimals;
        
        // Transfer 
        const transferTx = await transfer(connection, keypair, from_ata.address, to_ata.address, keypair, amount )

        // log
        console.log(`https://explorer.solana.com/tx/${transferTx}?cluster=devnet`)

    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();

// @to-do:: ofcourse you cant but still,  create a function to prove that you can't take ownership of other peoples ata.owner 