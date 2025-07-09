import wallet from "../../../Turbin3-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { 
    createMetadataAccountV3, 
    CreateMetadataAccountV3InstructionAccounts, 
    CreateMetadataAccountV3InstructionArgs,
    DataV2Args,
    findMetadataPda 
} from "@metaplex-foundation/mpl-token-metadata";
import { createSignerFromKeypair, signerIdentity, publicKey, none } from "@metaplex-foundation/umi";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { PublicKey } from "@solana/web3.js";

// Define our Mint address
const mint = publicKey("2zszKQQNpTtGAv419BRdtSLgbEavUJFZRXxPCffh935g")

// Create a UMI connection
const umi = createUmi('https://api.devnet.solana.com');
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);
umi.use(signerIdentity(createSignerFromKeypair(umi, keypair)));




// const metadataPda = findMetadataPda(umi, { mint });

const accounts: CreateMetadataAccountV3InstructionAccounts = {
  //metadata: metadataPda, // seems optional
  mint,
  mintAuthority: signer,
  payer: signer, // seems optional
  updateAuthority: signer, // seems optional
};

const data: DataV2Args = {
  name: "Burhan Khaja",
  symbol: "BK",
  uri: "https://avatars.githubusercontent.com/u/118989617?v=4", // github profile // Logo was not updated, there is some ntfs stuff going on | you will have to upload image to decentralized serves < lil research about that @note
  sellerFeeBasisPoints: 500,
  creators: null,     // optionals
  collection: null,
  uses: null,
};

const args: CreateMetadataAccountV3InstructionArgs = {
  data,
  isMutable: true,
   collectionDetails: none(), // absence of it causes errors // you can use `null` too
};

(async () => {
  try {
    const tx = createMetadataAccountV3(umi, {
      ...accounts,
      ...args,
    });

    const result = await tx.sendAndConfirm(umi);
    console.log("✅ Metadata created. Tx Signature:", bs58.encode(result.signature));
  } catch (e) {
    console.error("❌ Error creating metadata:", e);
  }
})();