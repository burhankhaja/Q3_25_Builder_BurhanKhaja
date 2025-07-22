import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";

describe("nft_staking", () => {
  // @audit :: use before() later to spawn new signer on each test entry , but if you happen to add any admin checks in lib.rs how do you make sure that before logic will cooperate ??? 
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.nftStaking as Program<NftStaking>;

  it("Initialize global config!", async () => {

    //@audit :: use BN and set them to realistic values later  
    let pointsPerStake = 0;
    let maxStake = 0;
    let freezePeriod = 0;

    await program.methods.initializeGlobalConfig(pointsPerStake, maxStake, freezePeriod).rpc();
  });

  //@test those b"seeds"" vs b"seeds"".as_ref thing >>>>> findSync() and rust part where- seeds- array creation part-- during cpi context ...... ///
});
