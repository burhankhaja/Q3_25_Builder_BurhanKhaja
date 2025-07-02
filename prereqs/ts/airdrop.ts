import { Connection, Keypair, LAMPORTS_PER_SOL } from
"@solana/web3.js"
import wallet from "./dev-wallet.json"

// import dev-wallet
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));
// devnet connection
const connection = new Connection("https://api.devnet.solana.com");

(async () => {
try {
// Airdrop 2 SOL to the wallet
    const txhash = await
connection.requestAirdrop(keypair.publicKey, 2 * LAMPORTS_PER_SOL);

console.log(`Success! Check out your TX here:
https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
} catch(e) {
console.error(`Oops, something went wrong: ${e}`)
}
})();

// first aidrop tx :: https://explorer.solana.com/tx/5B4mMtQrcfUtsQHCejsidomohD8smpEcXouDRwsHGBk7ciJNDdJBPJkFcBCjizDRyzYFHjJ9zHeNd665aZdHe5Cu?cluster=devnet
