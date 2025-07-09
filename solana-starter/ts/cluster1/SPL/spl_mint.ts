import { Keypair, PublicKey, Connection, Commitment, } from "@solana/web3.js";
import { getOrCreateAssociatedTokenAccount, mintTo } from '@solana/spl-token';
import wallet from "../../../Turbin3-wallet.json"


const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);

const token_decimals = 1_000_000n;

const ASSOCIATED_TOKEN_PROGRAM_ID = new PublicKey('ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL');

 const TOKEN_PROGRAM_ID = new PublicKey('TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA');


// Mint address
const mint = new PublicKey("2zszKQQNpTtGAv419BRdtSLgbEavUJFZRXxPCffh935g");
/** connection: Connection,
    payer: Signer,
    mint: PublicKey,
    owner: PublicKey,
    allowOwnerOffCurve = false,
    commitment?: Commitment,
    confirmOptions?: ConfirmOptions,
    programId = TOKEN_PROGRAM_ID,
    associatedTokenProgramId = ASSOCIATED_TOKEN_PROGRAM_ID, */
(async () => {
    try {
        // Create an ATA for the Token Authority : Turbin3-wallet.json
        // const ata = ???
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey,  true, undefined , undefined , TOKEN_PROGRAM_ID,ASSOCIATED_TOKEN_PROGRAM_ID ) // no need to use undefined etc ... just leave'm
        console.log(`Your ata is: ${ata.address.toBase58()}`); // CigwQPrxXWsdEih8D4gMJfW54qio7sW9tWxqNaEhcMpj

        // Mint to ATA
        // const mintTx = ???
        const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair, 100n * token_decimals);
        // const mintTx = await mintTo(connection, keypair, mint, ata.address, keypair, 1); //testing wei 

        console.log(`https://explorer.solana.com/tx/${mintTx}?cluster=devnet`)
        
    } catch(error) {
        console.log(`Oops, something went wrong: ${error}`)
    }
})()

