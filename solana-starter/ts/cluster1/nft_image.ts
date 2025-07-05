import wallet from "../../Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";
import { readFile } from "fs/promises";

// Setup
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {
    // Load local image
    const imageBuffer = await readFile("jeff.png");

    // create a generic file from image
    const imageFile = createGenericFile(imageBuffer, "jeff.png", {
      contentType: "image/png",
    });

    // Upload image
    const [imageUri] = await umi.uploader.upload([imageFile]);
    console.log("✅ Image uploaded:", imageUri); 
  } catch (error) {
    console.log("❌ Error uploading image:", error);
  }
})();

