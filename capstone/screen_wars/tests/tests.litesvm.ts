import { LiteSVM} from "litesvm";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
    TransactionInstruction,
    SystemProgram
} from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";

import { Buffer } from 'node:buffer';
import { BN } from "bn.js";
import { assert, timeStamp } from "node:console";


// helper imports
import * as methods from "./methods.helpers";
import * as state from "./state.helpers";
import * as clock from "./clock.helpers";
import * as txCheck from "./tx-check.helpers";
import * as constants from "./constants.helpers";


// Template :: tests
// it("", async() => {})


const programId = new PublicKey("4jqrWDfeR2RAzSPYNoiVq2dcVrZUrsp3ZWEPHehVwCtW");

//@dev :: Test from architechitectural diagram point of view >>>>> 
//@dev :: create separate helper for all these discriminators 


describe("screen_wars", () => {

    let svm: LiteSVM;
    let admin: Keypair;
    let jeff: Keypair;
    let shrinath: Keypair;
    let berg: Keypair;
    let globalPDA: PublicKey;
    let globalPDABump: any;



    //// Setup 

    beforeEach(async () => {
        svm = new LiteSVM();
        svm.addProgramFromFile(programId, "./target/deploy/screen_wars.so");

        // Set clock to current Unix timestamp (in seconds) for realism
        await clock.setInitialClock(svm);


        admin = new Keypair();
        jeff = new Keypair();
        shrinath = new Keypair();
        berg = new Keypair();

        svm.airdrop(admin.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(jeff.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(shrinath.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(berg.publicKey, BigInt(LAMPORTS_PER_SOL));

        [globalPDA, globalPDABump] = await state.getGlobalPDAAddressAndBump(programId);
    });

    //// Tests

    it("initialize state transition Integrity", async () => {


        await methods.initialize(svm, programId, admin, globalPDA);


        let globalState = await state.fetchGlobalPda(svm, programId,  {consoleLog: false });

        // // assertions
        assert(globalState.admin == admin.publicKey.toBase58(), "invalid admin");
        assert(globalState.treasury == globalPDA.toBase58(), "invalid treasury");
        assert(globalState.treasury_profits == 0, "Treasury profits must be 0")
        assert(globalState.challenge_ids == 1, "Initial challenge_ids value must be 1");
        assert(!globalState.challenge_creation_paused, "Challenge creation must be unpaused");
        assert(globalState.bump == globalPDABump, "Invalid bump stored");

    });


    it("Challenge creation is paused and unpaused correctly ", async () => {

        await methods.initialize(svm, programId, admin, globalPDA);


        //// pause
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, true, { logTxResult: false });


        // get global state
        let globalState = await state.fetchGlobalPda(svm, programId,  {consoleLog: false });


        // assert
        assert(globalState.challenge_creation_paused, "Expected Challenge creation to be paused")

        //// unpause
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, false, { logTxResult: false });

        // get global state after unpause
        let globalStateAfterUnpause = await state.fetchGlobalPda(svm, programId,  {consoleLog: false });


        // assert
        assert(!globalStateAfterUnpause.challenge_creation_paused, "challenge creation must be unpaused");
    });



    it("Only Admin can pause challenge creation", async () => {


        await methods.initialize(svm, programId, admin, globalPDA);


        // jeff tries to pause challenge creation 
        let jeffzPauseTx = await methods.setChallengeCreationPaused(svm, programId, jeff, globalPDA, true, { logTxResult: false });

        // caching global state
        let globalState = await state.fetchGlobalPda(svm, programId,  {consoleLog: false });


        // assertions
        assert(txCheck.isFailedTransaction(jeffzPauseTx), "Jeff must not be able to do success pause transaction, only admin is allowed");

        assert(!globalState.challenge_creation_paused, "expected unpaused, since only admin can do such changes")

    });

    it("Challenge creation by jeff, state transition Integrity", async () => {
    
        // initialize
        await methods.initialize(svm, programId, admin, globalPDA);

        //// create challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;

        // global pda initial state
        let initialGlobalPDAData = await state.fetchGlobalPda(svm, programId);

        // derive challengePda account address
        const [challengePDA, challengePDABump] = await state.getChallengePDAAddressAndBump(initialGlobalPDAData.challenge_ids, programId);

        // create challenge
        await methods.createChallenge(svm, programId, startTime , dailyTimer, jeff,  globalPDA, challengePDA, {logTxResult: false});

        // fetch challenge state and updated global state
        let challengePDAData = await state.fetchChallengePda(svm, programId, initialGlobalPDAData.challenge_ids, {consoleLog: false});

        let updatedGlobalPDAData = await state.fetchGlobalPda(svm, programId, {consoleLog: false});

        //// assertions
        // asserting global state
        assert(updatedGlobalPDAData.challenge_ids == initialGlobalPDAData.challenge_ids + 1, "Expected challengeIds to increment in globalPDA");
        
        // asserting challenge state
        assert(challengePDAData.challenge_id == initialGlobalPDAData.challenge_ids, "Expected id of challenge to be initial value of global state's challenge_ids");
        assert(challengePDAData.creator == jeff.publicKey.toBase58(), "Jeff must be the creator of the challenge");
        assert(challengePDAData.daily_timer == dailyTimer, "");
        assert(challengePDAData.start == startTime, "");
        assert(challengePDAData.end == startTime + constants.TWENTY_ONE_DAY_IN_SECONDS , "end");
        assert(challengePDAData.winner == constants.defaultPubkey.toBase58(), "Initial winner must be nullified with pubkey: 11111111111111111111111111111111"); 
        assert(challengePDAData.total_participants == 0, "There must never be any participant registered on creation");
        assert(challengePDAData.bump == challengePDABump, "Correct challengePda bump must be stored");

    });

    it("challenge creation fails after paused by admin", async() => {

        await methods.initialize(svm, programId, admin, globalPDA);
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, true, { logTxResult: false });

        // create challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let initialGlobalPDAData = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(initialGlobalPDAData.challenge_ids, programId);
        let result =  await methods.createChallenge(svm, programId, startTime , dailyTimer, jeff,  globalPDA, challengePDA, {logTxResult: false}); //@dev for debugging , try logTxResult: true

        // assertion
        assert(txCheck.isFailedTransaction(result), "Expected transaction to fail");

    });

   
    // Test :: create Challenge failure cases: 
    // challenge start > now + 1 day ? 
    // challenge start < now + 7 days ? 
    // daily challenge time < 2 hours ?
    // @audit :: false -> tx must revert!

    // @later :: must
    // create userflows.helpers.ts  // userStories.helpers.ts :: such that , initialize by admin, challenge creation by jeff-- is abstracted

    // join
    // sync :: refactoring
    // sync :: normal test
    // withdraw
    // claim_winner_position

    it("join challenge, state Transition Integrity", async() => {
        // initialize
        await methods.initialize(svm, programId, admin, globalPDA);

        // create challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let globalPDAData = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(globalPDAData.challenge_ids, programId);
        await methods.createChallenge(svm, programId, startTime , dailyTimer, jeff,  globalPDA, challengePDA, {logTxResult: false});
        let initialChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, {consoleLog: false});

        //// join challenge
        let _challengeId = initialChallengePDAData.challenge_id;
        let [shrinathUserPDA, shrinathUserPDABump] = await state.getUserPDAAddressAndBump(shrinath.publicKey, programId);

        await methods.joinChallenge(svm, programId, _challengeId, shrinath, shrinathUserPDA, globalPDA, challengePDA, {logTxResult: false});

        // post join state
        let updatedChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, {consoleLog: false});

        let shrinathUserPDAData = await state.fetchUserPda(svm, programId, shrinath.publicKey, {consoleLog: false});



        //// assertions
        // challengePDA assertion
        assert(updatedChallengePDAData.total_participants == initialChallengePDAData.total_participants + 1, "Expected total participants to increment after joined by user");

        // userPDA assertions
        assert(shrinathUserPDAData.user == shrinath.publicKey.toBase58(), "shrinath must be stored as user in his userPDA");
        assert(shrinathUserPDAData.challenge_id == updatedChallengePDAData.challenge_id, "Expected userPDA to store enrolled challenge's id");
        assert(shrinathUserPDAData.locked_balance == 0, "On joining, locked balance must always be 0");
        assert(shrinathUserPDAData.streak == 0, "user will always have 0 streak on joining");
        assert(shrinathUserPDAData.bump == shrinathUserPDABump, "Expected shrinath's userpda to be stored correctly");
    });



});