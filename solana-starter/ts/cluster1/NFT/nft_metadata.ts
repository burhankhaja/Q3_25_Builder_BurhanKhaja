import wallet from "../../../Turbin3-wallet.json";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import {
  createGenericFile,
  createSignerFromKeypair,
  signerIdentity,
} from "@metaplex-foundation/umi";
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys";

// Setup
const umi = createUmi("https://api.devnet.solana.com");
const keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
  try {

    // insert uploaded image uri from irys
    const imageUri = "https://gateway.irys.xyz/CSJHiCGBo4AspaFu9nh5VJtNW7DebWbmCJYmnSvGUFAx";

    const metadata = {
      name: "banned by Jeff",
      symbol: "BANN",
      description: "Dedicated to Jeff, the ultimate gatekeeper of Turbin3. One typo, and you're history",
      image: imageUri,
      attributes: [
        {
          trait_type: "Disqualification",
          value: "99",
        },
      ],
      properties: {
        files: [
          {
            uri: imageUri,
            type: "image/png",
          },
        ],
      },
      creators: [
        {
          address: signer.publicKey,
          share: 100,
          verified: true,
        },
      ],
    };
    

    // convert metadata to buffer and upload to iyrs server
    const metadataJson = Buffer.from(JSON.stringify(metadata));
    const jsonFile = createGenericFile(metadataJson, "metadata.json", {
      contentType: "application/json",
    });
    
    const [uri] = await umi.uploader.upload([jsonFile]);
    console.log("✅ Metadata URI:", uri);  // jeff.png : https://gateway.irys.xyz/EciVvwZ338ainzop3hC4UtQKWiWTNiWaeyxLVmUhTS24
  } catch (error) {
    console.log("❌ Error uploading metadata:", error);
  }
})();

/**   
  //@idea-later ::  WHY NOT CREATE JEFF NFT WITH THANOS THEME WITH INFINITY STONE TS && RUST AND CLICKING FINGERS DISQUALIFYING COHERT APPLICATNS
   
  // jeff's address :: BvhV49WPYBbzPu8Fpy8YnPnwhNWLbm9Vmdj2T5bNSotS 
    
*/