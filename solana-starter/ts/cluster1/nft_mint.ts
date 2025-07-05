import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createSignerFromKeypair,
  signerIdentity,
  generateSigner,
  percentAmount,
  some,
} from "@metaplex-foundation/umi";
import { createNft, mplTokenMetadata } from "@metaplex-foundation/mpl-token-metadata";

import wallet from "../../Turbin3-wallet.json";
import base58 from "bs58";

// Setup
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const myKeypairSigner = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(myKeypairSigner));
umi.use(mplTokenMetadata());

const mint = generateSigner(umi);

(async () => {
  try {

    const uri = " https://gateway.irys.xyz/EciVvwZ338ainzop3hC4UtQKWiWTNiWaeyxLVmUhTS24";

    const tx = await createNft(umi, {
      mint,
      name: "banned by Jeff",
      symbol: "BANN",
      uri,
      sellerFeeBasisPoints: percentAmount(5), // 5%
      creators: some([
        {
          address: myKeypairSigner.publicKey,
          verified: true,
          share: 100,
        },
      ]),
      isMutable: true,
    }).sendAndConfirm(umi);

    const signature = base58.encode(tx.signature);
    console.log(`✅ NFT Minted! Tx: https://explorer.solana.com/tx/${signature}?cluster=devnet`);
    console.log("🧾 Mint Address:", mint.publicKey.toString());
  } catch (e) {
    console.error("❌ Minting failed:", e);
  }
})();



/**
 
✅ NFT Minted! Tx: https://explorer.solana.com/tx/CvjHco2QBQK3BFWrC6U3UidfxdTyLx2iuC9TL533NtT1Y6ZeGryyUAbd7mUVx3wkdmi5AoySNrrztDf3VYro6tn?cluster=devnet
🧾 Mint Address: AvgMiejkh5Wod8QEvqcbfRur3h1NAruiBVLjTb6gZgUj
   
    https://explorer.solana.com/address/AvgMiejkh5Wod8QEvqcbfRur3h1NAruiBVLjTb6gZgUj?cluster=devnet

*/
