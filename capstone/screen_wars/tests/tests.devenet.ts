import { assert } from "chai";
import * as anchor from "@coral-xyz/anchor";
import * as constants from "./constants.helpers";
import { ScreenWars } from "../target/types/screen_wars";
// import wallet from "../../../wallets/Turbin3-wallet.json";

describe("Screen Wars Devent Tests", () => {

    let provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    const deployer = anchor.web3.Keypair.fromSecretKey(new Uint8Array(provider.wallet.payer.secretKey)); // anchor.web3.Keypair.fromSecretKey(new Uint8Array(wallet)); ----> Turbin3-wallet.json
    const connection = new anchor.web3.Connection("https://api.devnet.solana.com");
    const program = anchor.workspace.ScreenWars as anchor.Program<ScreenWars>; // 4jqrWDfeR2RAzSPYNoiVq2dcVrZUrsp3ZWEPHehVwCtW


    // realtime Devnet PDAs from past interactions
    let Deployer = new anchor.web3.PublicKey("5gLcyi3EZK56aVc6XEurDiXvk74s9xMHKhFAsAAyZfLg");
    let globalPDA = new anchor.web3.PublicKey("B7mvKHJjTorTSBQjX2S87hWFz2AmBhujs25Pgm6DXXEL");
    let firstChallengePDA = new anchor.web3.PublicKey("A7GMvZyruaTejfpR9NmDSvez8KN2qpkN9DftaEwyKDpR");
    let firstUserPDA = new anchor.web3.PublicKey("E4xQAsveYbUgRvTsrEykx1MQBqRrka7ViHBWQjtqTRuT");

    // fetch all realtime state pda's data from devnet
    let globalPdaData;
    let firstChallengePdaData;
    let firstUserPdaData;

    beforeEach(async () => {
        globalPdaData = await program.account.global.fetch(globalPDA);
        firstChallengePdaData = await program.account.challenge.fetch(firstChallengePDA);
        firstUserPdaData = await program.account.user.fetch(firstUserPDA);
    });


    //// GLOBAL PDA STATE
    it("#DEVNET: Challenge creation should be paused", async () => {
        assert(globalPdaData.challengeCreationPaused, "Challenge creation must be paused as previously set by the admin");
    });

    it("#DEVNET: Next available challenge ID should be one greater than the total challenges created", async () => {
        assert(globalPdaData.challengeIds == firstChallengePdaData.challengeId + 1, "Global PDA should store 2 as the next challenge ID since only one challenge exists on Devnet")
    });

    it("#DEVNET: Program's Admin must equal the original Deployer", async () => {
        assert(globalPdaData.admin == Deployer.toBase58(), "Admin must be the original Deployer who initialized the program");
    });

    it("#DEVNET: globalPda should serve as the program treasury with 0 profits before any challenge ends", async () => {
        assert(globalPdaData.treasury.toBase58() == globalPDA.toBase58(), "globalPda must be treasury");
        assert(globalPdaData.treasuryProfits == 0, "treasury profits must start at 0");
    });

    //// CHALLENGE PDA STATE
    it("#DEVNET: The first challenge must have ID 1", async () => {
        assert(firstChallengePdaData.challengeId == 1, "The first challenge's ID should be exactly 1 and so on");
    });

    it("#DEVNET: All challenges must end exactly 21 days after start", async () => {
        let startTime = Number(firstChallengePdaData.start);
        let endTime = Number(firstChallengePdaData.end);

        assert(endTime == startTime + constants.TWENTY_ONE_DAY_IN_SECONDS, "Each challenge must end exactly 21 days after its start; first challenge must follow this rule as well");
    });


    it("#DEVNET: First challenge must have default winner and zero winner streak before winner position claiming phase", async () => {
        assert(firstChallengePdaData.winner == constants.defaultPubkey.toBase58(), "Winner must be the default pubkey to indicate no winner has been appointed yet");
        assert(firstChallengePdaData.winnerStreak == 0, "Winner streak must be 0 since no winner exists yet");
    });

    //// USER PDA STATE
    it("#DEVNET: First participant of challengeId 1 should have 0 locked balance and 0 streak since the challenge hasn’t started on devnet", async () => {
        assert(firstUserPdaData.lockedBalance == 0, "locked balance must be 0");
        assert(firstUserPdaData.streak == 0, "streak must be 0");
    });

    it("#DEVNET: A participant's PDA must store the challengeId they enrolled in", async () => {
        assert(firstUserPdaData.challengeId == 1, "first user enrolled in challengeId 1, so PDA must store 1");
    });


    /*
        ///////////////////////////////////////
        //  
        //       M E T H O D     C A L L S
        //
        ///////////////////////////////////////
    
    
        // INITIALIZE
        it.skip("initialize", async () => {
            const txHash = await program.methods.initialize().signers([deployer]).rpc();
            console.log(txHash);
        });
    
        // CREATE CHALLENGE
        it.skip("create first challenge", async () => {

            let now = await connection.getBlockTime(await connection.getSlot());
            let startTime = new BN(now + constants.ONE_DAY_IN_SECONDS + 1); // @note :: anchor automatically handles toArrayLike(Buffer , "encoding", "bytes")
            let dailyTimer = new BN(constants.ONE_HOUR_IN_SECONDS);
            const txHash = await program.methods.createChallenge(startTime, dailyTimer).accountsPartial({
                creator: deployer.publicKey,
                global: globalPDA,
            }).signers([deployer]).rpc();
    
            console.log("create challenge tx : ", txHash);

        });
    
        // JOIN CHALLENGE
        it.skip("Join first challenge", async () => {
            let challengeId = 1;
            let txhash = await program.methods.joinChallenge(challengeId).accountsPartial({
                user: deployer.publicKey,
                global: globalPDA,
                challenge: firstChallengePDA
            }).signers([deployer]).rpc();
    
            console.log(`https://explorer.solana.com/tx/${txhash}?cluster=devnet`);
        });
    
        // PAUSE CHALLENGE CREATION
        it.skip("pause challenge creation", async () => {
            let pause = true;
            let txHash = await program.methods.setChallengeCreationPaused(pause).accounts({
                admin: deployer.publicKey,
                global: globalPDA,
            }).signers([deployer]).rpc();
    
            console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
        });
    
        // UNPAUSE CHALLENGE CREATION
        it.skip("unpause challenge creation", async () => {
            let pause = false;
            let txHash = await program.methods.setChallengeCreationPaused(pause).accounts({
                admin: deployer.publicKey,
                global: globalPDA,
            }).signers([deployer]).rpc();
    
            console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
        });
    
        // CREATE CHALLENGE AFTER PAUSE MUST FAIL
        it.skip("challenges cant be created after pause", async () => {
            let pause = true;
            let txHash = await program.methods.setChallengeCreationPaused(pause).accounts({
                admin: deployer.publicKey,
                global: globalPDA,
            }).signers([deployer]).rpc();
    
            console.log(`https://explorer.solana.com/tx/${txHash}?cluster=devnet`);
    
    
            //==================================
            // creating challenge during pause state
            const ONE_HOUR_IN_SECONDS = 60 * 60;
            const ONE_DAY_IN_SECONDS = 24 * ONE_HOUR_IN_SECONDS;
    
    
    
            let now = await connection.getBlockTime(await connection.getSlot());
            let startTime = new BN(now + constants.ONE_DAY_IN_SECONDS * 3); // @note :: anchor automatically handles toArrayLike(Buffer , "encoding", "bytes")
            let dailyTimer = new BN(constants.ONE_HOUR_IN_SECONDS / 2);

            const createHash = await program.methods.createChallenge(startTime, dailyTimer).accountsPartial({
                creator: deployer.publicKey,
                global: globalPDA,
            }).signers([deployer]).rpc();
    
            console.log("create challenge tx : ", createHash);
        });
    */

});

