import { LiteSVM } from "litesvm";
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
import * as userflows from "./userflows.helpers";


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

    it("should maintain state integrity when initializing", async () => {


        await methods.initialize(svm, programId, admin, globalPDA);


        let globalState = await state.fetchGlobalPda(svm, programId, { consoleLog: false });

        // // assertions
        assert(globalState.admin == admin.publicKey.toBase58(), "invalid admin");
        assert(globalState.treasury == globalPDA.toBase58(), "invalid treasury");
        assert(globalState.treasury_profits == 0, "Treasury profits must be 0")
        assert(globalState.challenge_ids == 1, "Initial challenge_ids value must be 1");
        assert(!globalState.challenge_creation_paused, "Challenge creation must be unpaused");
        assert(globalState.bump == globalPDABump, "Invalid bump stored");

    });


    it("should correctly pause and unpause challenge creation ", async () => {

        await methods.initialize(svm, programId, admin, globalPDA);


        //// pause
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, true, { logTxResult: false });


        // get global state
        let globalState = await state.fetchGlobalPda(svm, programId, { consoleLog: false });


        // assert
        assert(globalState.challenge_creation_paused, "Expected Challenge creation to be paused")

        //// unpause
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, false, { logTxResult: false });

        // get global state after unpause
        let globalStateAfterUnpause = await state.fetchGlobalPda(svm, programId, { consoleLog: false });


        // assert
        assert(!globalStateAfterUnpause.challenge_creation_paused, "challenge creation must be unpaused");
    });



    it("should allow only the admin to pause challenge creation", async () => {


        await methods.initialize(svm, programId, admin, globalPDA);


        // jeff tries to pause challenge creation 
        let jeffzPauseTx = await methods.setChallengeCreationPaused(svm, programId, jeff, globalPDA, true, { logTxResult: false });

        // caching global state
        let globalState = await state.fetchGlobalPda(svm, programId, { consoleLog: false });


        // assertions
        assert(txCheck.isFailedTransaction(jeffzPauseTx), "Jeff must not be able to do success pause transaction, only admin is allowed");

        assert(!globalState.challenge_creation_paused, "expected unpaused, since only admin can do such changes")

    });

    it("should correctly handle state transitions when Jeff creates a challenge", async () => {

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
        await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });

        // fetch challenge state and updated global state
        let challengePDAData = await state.fetchChallengePda(svm, programId, initialGlobalPDAData.challenge_ids, { consoleLog: false });

        let updatedGlobalPDAData = await state.fetchGlobalPda(svm, programId, { consoleLog: false });

        //// assertions
        // asserting global state
        assert(updatedGlobalPDAData.challenge_ids == initialGlobalPDAData.challenge_ids + 1, "Expected challengeIds to increment in globalPDA");

        // asserting challenge state
        assert(challengePDAData.challenge_id == initialGlobalPDAData.challenge_ids, "Expected id of challenge to be initial value of global state's challenge_ids");
        assert(challengePDAData.creator == jeff.publicKey.toBase58(), "Jeff must be the creator of the challenge");
        assert(challengePDAData.daily_timer == dailyTimer, "daily challenge timer must be what challenge creator intended");
        assert(challengePDAData.start == startTime, "expected challenge pda start time to be stored as per challenge creator's startTime");
        assert(challengePDAData.end == startTime + constants.TWENTY_ONE_DAY_IN_SECONDS, "Expected challenge to end 21 days after now");
        assert(challengePDAData.winner == constants.defaultPubkey.toBase58(), "Initial winner must be nullified with pubkey: 11111111111111111111111111111111");
        assert(challengePDAData.total_participants == 0, "There must never be any participant registered on creation");
        assert(challengePDAData.bump == challengePDABump, "Correct challengePda bump must be stored");

    });

    it("should prevent challenge creation after the admin has paused it", async () => {

        await methods.initialize(svm, programId, admin, globalPDA);
        await methods.setChallengeCreationPaused(svm, programId, admin, globalPDA, true, { logTxResult: false });

        // create challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let initialGlobalPDAData = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(initialGlobalPDAData.challenge_ids, programId);
        let result = await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false }); //@dev for debugging , try logTxResult: true

        // assertion
        assert(txCheck.isFailedTransaction(result), "Expected transaction to fail");

    });


    // Test :: create Challenge failure cases: 
    // challenge start > now + 1 day ? 
    // challenge start < now + 7 days ? 
    // daily challenge time < 2 hours ?
    // @audit :: false -> tx must revert!

    it("should maintain correct state transitions when a user joins a challenge", async () => {
        // initialize
        await methods.initialize(svm, programId, admin, globalPDA);

        // create challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let globalPDAData = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(globalPDAData.challenge_ids, programId);
        await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });
        let initialChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, { consoleLog: false });

        //// join challenge
        let _challengeId = initialChallengePDAData.challenge_id;
        let [shrinathUserPDA, shrinathUserPDABump] = await state.getUserPDAAddressAndBump(shrinath.publicKey, programId);

        await methods.joinChallenge(svm, programId, _challengeId, shrinath, shrinathUserPDA, globalPDA, challengePDA, { logTxResult: false });

        // post join state
        let updatedChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, { consoleLog: false });

        let shrinathUserPDAData = await state.fetchUserPda(svm, programId, shrinath.publicKey, { consoleLog: false });



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


    it("should fail to join a challenge that has already started", async () => {
        //// initialize & create challenge
        await methods.initialize(svm, programId, admin, globalPDA);
        //
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let globalPDAData = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(globalPDAData.challenge_ids, programId);

        // create challenge
        await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });

        // fetch challengePDA  
        let ChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, { consoleLog: false });

        //// time travel to a period when challenge has started
        let challengeStartingTime = ChallengePDAData.start;
        clock.setTimeStamp(svm, challengeStartingTime);


        // //// join challenge
        let _challengeId = globalPDAData.challenge_ids;
        let [shrinathUserPDA] = await state.getUserPDAAddressAndBump(shrinath.publicKey, programId);
        let result = await methods.joinChallenge(svm, programId, _challengeId, shrinath, shrinathUserPDA, globalPDA, challengePDA, { logTxResult: false });

        //// assertions
        assert(txCheck.isFailedTransaction(result), "Expected tx to fail, because challenges can't be joined after they start");

    });

    it.skip("users cant join 2 challenges at a time", async () => {
        //@audit :: @dev :: @note
        // Test users can only join one challenge at a time::::: must need of challenge creation abstraction to reduce code size :::::
        /// @later :::: lets do it later ::: create 2 challenges, then user joins one and attempts to join another to which tx must fail

    });



    it("should correctly transition state on the initial SyncLock after a successful daily challenge", async () => {
        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);

        // time travel to when challenge starts
        let challengeId = 1;
        let challengeStartingTime = (await state.fetchChallengePda(svm, programId, challengeId)).start;
        clock.setTimeStamp(svm, challengeStartingTime);

        let treasurySOLBalanceBefore = svm.getBalance(globalPDA); // since address(globalPDA) == globalPDA.treasury
        let shrinathSOLBalanceBefore = svm.getBalance(shrinath.publicKey);

        // syncLock
        await methods.syncAndLock(svm, programId, challengeId, shrinath, { logTxResult: false });

        let treasurySOLBalanceAfter = svm.getBalance(globalPDA); // since address(globalPDA) == globalPDA.treasury
        let shrinathSOLBalanceAfter = svm.getBalance(shrinath.publicKey);
        let txFeeInLamports = BigInt(5_000);

        // fetch shrinaths user pda and global pda states
        let shrinathUserPDAData = await state.fetchUserPda(svm, programId, shrinath.publicKey, { consoleLog: false });
        let globalPDAData = await state.fetchGlobalPda(svm, programId, { consoleLog: false });


        // userPda
        assert(shrinathUserPDAData.streak == 1, "expected streak to increase after initial sync lock after successfully passing challenge for the day");
        assert(shrinathUserPDAData.locked_balance == 10_000_000, "after initial sync, 10 million lamports must be stored as locked_balance for user, (assuming successful daily challenge completion)")
        assert(shrinathSOLBalanceBefore == shrinathSOLBalanceAfter + BigInt(10_000_000) + txFeeInLamports, "10 Million lamports of SOL must be taken from user while syncLocking for the day")

        //globalPda
        assert(globalPDAData.treasury_profits == 0, "Expected treasury profits to stay unchanged after user deposit via sync_and_lock");
        assert(treasurySOLBalanceAfter == treasurySOLBalanceBefore + BigInt(10_000_000), "Expected 10Million Sol lamports to be deposited in globalPda address (which is : globalPDAData.treasury)")

    });

    it("should penalize the user for failing the first daily challenge by deducting only the daily lamports, without updating balance or streak", async () => {

        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);

        // time travel to when challenge starts
        let challengeId = 1;
        let challengeStartingTime = (await state.fetchChallengePda(svm, programId, challengeId)).start;
        clock.setTimeStamp(svm, challengeStartingTime);

        let txFeeInLamports = 5_000;
        let shrinathSOLBalanceBefore = svm.getBalance(shrinath.publicKey);
        let treasurySOLBalanceBefore = svm.getBalance(globalPDA);


        // sync
        await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false,
            debug: {
                user_passed: false, // Simulating case : user failed challenge for the day!
                days_not_synced: 0,
                synced_today: false
            }
        });

        let treasurySOLBalanceAfter = svm.getBalance(globalPDA);
        let shrinathSOLBalanceAfter = svm.getBalance(shrinath.publicKey);
        let shrinathUserPDAData = await state.fetchUserPda(svm, programId, shrinath.publicKey, { consoleLog: false });
        let challengePDAData = await state.fetchChallengePda(svm, programId, challengeId, { consoleLog: false });
        let globalPDAData = await state.fetchGlobalPda(svm, programId, { consoleLog: false });


        //// assertions
        // shrinath assertions
        assert(shrinathSOLBalanceAfter == shrinathSOLBalanceBefore - BigInt(10_000_000) - BigInt(txFeeInLamports), "10 million lamports must be taken from user");
        assert(shrinathUserPDAData.streak == 0, "streak must stay 0 on failing initial daily challenge");
        assert(shrinathUserPDAData.locked_balance == 0, "locked_balance should remain zero when failing initial day of challenge");

        // challengePda assertions
        assert(challengePDAData.total_slashed == 10_000_000, "user's daily deposit lamports should be recorded as the slashed amount for the challenge");

        // globalPda assertions
        assert(globalPDAData.treasury_profits == 0, "treasury profits should not increase solely from slashing users");
        assert(treasurySOLBalanceAfter == treasurySOLBalanceBefore + BigInt(10_000_000), "treasury SOL balance should increase by the slashed lamports from the user"); // globalPDA acts as treasury
    });

    it.skip("should penalize the user for failing any daily challenge after the first by deducting the daily lamports plus 25% of the locked balance, and resetting their streak", async () => { });

    it.skip("should not be able to sync if the challenge time has not started", async () => { });

    it.skip("cant sync after end", async () => { });
    it.skip("cant sync after already synced", async () => { });


    it("should allow the highest streak holder to claim the winner position and prevent lower streak overrides", async () => {
        let challengeId = 1;

        //// skip to challenge end : where berg has more streak than shrinath
        //// and total slashed is 10 million lamports (from shrinath's account)
        await userflows.teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, challengeId);

        //// first shrinath claims winnership
        await methods.claimWinnerPosition(svm, programId, challengeId, shrinath, { logTxResult: false });

        let challengeStateAfterShrinathTx = await state.fetchChallengePda(svm, programId, challengeId, { consoleLog: false });


        //// then berg over-rides him as winner (since berg has more streak)
        await methods.claimWinnerPosition(svm, programId, challengeId, berg, { logTxResult: false });

        let challengeStateAfterBergTx = await state.fetchChallengePda(svm, programId, challengeId, { consoleLog: false });


        //// then shrinath again tries to over-ride him but fails
        let shrinathOverridingBergTx = await methods.claimWinnerPosition(svm, programId, challengeId, shrinath, { logTxResult: false });


        //// fetch user pdas
        let shrinathUserPDAData = await state.fetchUserPda(svm, programId, shrinath.publicKey, { consoleLog: false })
        let bergUserPDAData = await state.fetchUserPda(svm, programId, berg.publicKey, { consoleLog: false })

        //// assertions

        // asserting initial shrinath tx
        assert(challengeStateAfterShrinathTx.winner.toBase58() == shrinathUserPDAData.user.toBase58(), "initial winner must be shrinath");
        assert(challengeStateAfterShrinathTx.winner_streak == shrinathUserPDAData.streak, "since shrianth is the initial winner, his streak must be winner streak for the challenge initially");

        // asserting berg's tx 
        assert(challengeStateAfterBergTx.winner.toBase58() == bergUserPDAData.user.toBase58(), "berg must override shrinath as winner, since he has higher streak");
        assert(challengeStateAfterBergTx.winner_streak == bergUserPDAData.streak, "since berg has overriden shrinath, his streak must be set as benchmark for the challenge");

        // asserting shrianth's another tx 
        assert(txCheck.isFailedTransaction(shrinathOverridingBergTx), "user with lower streak must not be able to override users with higher streak");

    });

    it("should prevent claiming the winner position before the challenge has ended", async () => {
        let challengeId = 1;

        // skip to challenge end : berg more streak than shrinath :: total slashed == 10 mill lamports
        await userflows.teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // since teleportToChallengeEnd also sets time to challengeEnd, we will need to go back before the challenge was still active
        let beforeChallengeEnd = await clock.now(svm) - 1;
        await clock.setTimeStamp(svm, beforeChallengeEnd);

        let txResult = await methods.claimWinnerPosition(svm, programId, challengeId, berg, { logTxResult: false });

        assert(txCheck.isFailedTransaction(txResult), "user must not be able to claim winner position before the challenge has ended");
    })

    it("should prevent claiming the winner position after the contention period has passed", async () => {
        let challengeId = 1;

        await userflows.teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // time travel to reward claiming phase
        let afterContentionPeriod = (await clock.now(svm) + (constants.ONE_DAY_IN_SECONDS * 5)); // since contention period spawn 5 days after end , and now is the 
        await clock.setTimeStamp(svm, afterContentionPeriod);

        ////
        let txResult = await methods.claimWinnerPosition(svm, programId, challengeId, berg, { logTxResult: false });

        //// assertion
        assert(txCheck.isFailedTransaction(txResult), "user must not be able to claim winner position after contention period");

    })

    it.skip("fail: non enrolled user tries to claim", async () => { });


    //@
    it("withdraw test", async () => {
        let challengeId = 1;

        // teleport to reward claiming phase
        await userflows.teleportToRewardClaimingPhase_________bergDeclaredWinner(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // withdraw
        // claimRewardAsWinner
        // claimRewardAsCreator
        
        // then withdraw + claimRewardAs Winner ====> take_protocol_profits





        //// fetch states
        await state.fetchUserPda(svm, programId, berg.publicKey, { consoleLog: true })
        await state.fetchUserPda(svm, programId, shrinath.publicKey, { consoleLog: true })
        await state.fetchChallengePda(svm, programId, 1, { consoleLog: true });


    });


    // it("", async() => {});



});