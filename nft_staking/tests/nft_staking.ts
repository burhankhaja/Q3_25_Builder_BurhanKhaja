import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NftStaking } from "../target/types/nft_staking";

describe("nft_staking", () => {
  // Configure the client to use the local cluster.
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
});
