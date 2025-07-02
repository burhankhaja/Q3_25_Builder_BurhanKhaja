import { Transaction, SystemProgram, Connection, Keypair,
LAMPORTS_PER_SOL, sendAndConfirmTransaction, PublicKey } from
"@solana/web3.js"
import wallet from "./dev-wallet.json"


// Import our dev wallet keypair from the wallet file
const from = Keypair.fromSecretKey(new Uint8Array(wallet));
// Define our Turbin3 public key
const to = new
PublicKey("5gLcyi3EZK56aVc6XEurDiXvk74s9xMHKhFAsAAyZfLg");
const connection = new Connection("https://api.devnet.solana.com");
(async () => {
  try {
// Get balance of dev wallet
    const balance = await connection.getBalance(from.publicKey);

// Create a test transaction to calculate fees
    let transaction = new Transaction().add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance, 
      })
    );
    transaction.recentBlockhash = (await connection.getLatestBlockhash("confirmed")).blockhash;
    transaction.feePayer = from.publicKey;

// Calculate exact fee rate to transfer entire SOL amount outof account minus fees
 const fee = (await connection.getFeeForMessage(transaction.compileMessage(), "confirmed")).value || 0;

// Remove our transfer instruction to replace it
    transaction.instructions.pop();
// Now add the instruction back with correct amount of
    
    transaction.add(
      SystemProgram.transfer({
        fromPubkey: from.publicKey,
        toPubkey: to,
        lamports: balance - fee,
      })
    );

    //// Sign transaction, broadcast, and confirm

    const signature = await sendAndConfirmTransaction(connection, transaction, [from]);

    console.log(`Success! Check out your TX here:`);
    console.log(`https://explorer.solana.com/tx/${signature}?cluster=devnet`);
  } catch (e) {
console.error(`Oops, something went wrong: ${e}`);
}
})();


// first 0.1 sol transfer :: https://explorer.solana.com/tx/3kiKc9ahkgAmnu8rxcYEqqSAdyabqsmG1hU3PTadRy6K4TRjATNRZNAhVB4gH6S26t1uk17XtDpjCuJWvmGqW3na?cluster=devnet
// final full transfer :: https://explorer.solana.com/tx/5ZvDvEV9Jo92728RCG5A53boE8jpb7PZpXhSL3WVmQVuuMhtp5q3kRGpXGnyrUuhs7toB78GkpmHXELZVktoJz9A?cluster=devnet