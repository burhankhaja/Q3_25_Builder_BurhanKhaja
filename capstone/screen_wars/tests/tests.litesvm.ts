import { LiteSVM } from "litesvm";
import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
} from "@solana/web3.js";
import { assert } from "node:console";


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
    let globalPDA: PublicKey;
    let globalPDABump: any;

    let admin: Keypair;
    let apaar: Keypair;
    let berg: Keypair;
    let jeff: Keypair;
    let shrinath: Keypair;



    //// Setup 

    beforeEach(async () => {
        svm = new LiteSVM();
        svm.addProgramFromFile(programId, "./target/deploy/screen_wars.so");

        // Set clock to current Unix timestamp (in seconds) for realism
        await clock.setInitialClock(svm);


        admin = new Keypair();
        apaar = new Keypair();
        berg = new Keypair();
        jeff = new Keypair();
        shrinath = new Keypair();

        svm.airdrop(admin.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(apaar.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(berg.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(jeff.publicKey, BigInt(LAMPORTS_PER_SOL));
        svm.airdrop(shrinath.publicKey, BigInt(LAMPORTS_PER_SOL));

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

    it("should reject challenge creation if the start time is less than twenty-four hours from now, more than seven days from now, or if the daily timer is longer than two hours", async () => {
        //// initialize
        await methods.initialize(svm, programId, admin, globalPDA);

        //// create challenge
        let challengeId = 1; // since globalPDAData.challenge_ids == 1
        let [challengePDA] = await state.getChallengePDAAddressAndBump(challengeId, programId);
        let normalDailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let dailyTimerMoreThanTwoHours = (constants.ONE_HOUR_IN_SECONDS * 2) + 1;
        let startTimeLessThanOneDayFromNow = await clock.now(svm);
        let normalStartTimeWithOneDayDelay = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let startTimeMoreThanSevenDayFromNow = await clock.now(svm) + (constants.ONE_DAY_IN_SECONDS * 7);

        // creating challenge without giving delay of atleast one day
        let challengeWithoutOneDayDelayTx = await methods.createChallenge(svm, programId, startTimeLessThanOneDayFromNow, normalDailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });

        // creating challenge without giving delay of atleast one day
        let challengeWithMoreThanSevenDayDelayTx = await methods.createChallenge(svm, programId, startTimeMoreThanSevenDayFromNow, normalDailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });

        // creating challenge with daily timer more than two hours
        let challengeWithDailyTimerMoreThanTwoHourTx = await methods.createChallenge(svm, programId, normalStartTimeWithOneDayDelay, dailyTimerMoreThanTwoHours, jeff, globalPDA, challengePDA, { logTxResult: false });

        //// assertions
        assert(txCheck.isFailedTransaction(challengeWithoutOneDayDelayTx), "Challenge creation should fail if start_time is less than 24h from now");
        assert(txCheck.isFailedTransaction(challengeWithMoreThanSevenDayDelayTx), "Challenge creation should fail if start_time exceeds 7 days from now");
        assert(txCheck.isFailedTransaction(challengeWithDailyTimerMoreThanTwoHourTx), "Challenge creation should fail if daily timer exceeds 2 hours");
    });

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


    it("should prevent a user from joining more than one challenge at a time", async () => {

        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);


        // jeff creates another challenge
        let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
        let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
        let globalPDADataBefore = await state.fetchGlobalPda(svm, programId);
        const [challengePDA] = await state.getChallengePDAAddressAndBump(globalPDADataBefore.challenge_ids, programId);
        await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });

        // berg tries to join challenge no 2 after already enrolled in challenge 1
        let [bergUserPDA] = await state.getUserPDAAddressAndBump(berg.publicKey, programId);

        let bergSecondChallengeJoiningTx = await methods.joinChallenge(svm, programId, globalPDADataBefore.challenge_ids, berg, bergUserPDA, globalPDA, challengePDA, { logTxResult: false });

        assert(txCheck.isFailedTransaction(bergSecondChallengeJoiningTx), "a user must not be allowed to participate in more than one challenge at the same time");

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




    it("should penalize the user for failing any daily challenge after the first by deducting the daily lamports plus 25% of the locked balance, and resetting their streak", async () => {

        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);

        // time travel to when challenge starts
        let challengeId = 1;
        let challengeStartingTime = (await state.fetchChallengePda(svm, programId, challengeId)).start;
        clock.setTimeStamp(svm, challengeStartingTime);

        // shrinath passes on first day
        await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: true,
                days_not_synced: 0,
                synced_today: false,
            }
        });

        let shrinathSolBalanceBeforeFailDay = Number(svm.getBalance(shrinath.publicKey));
        let shrianthLockedBalanceBeforeFailDay = Number((await state.fetchUserPda(svm, programId, shrinath.publicKey)).locked_balance);

        let tommorrow = Number(await clock.now(svm)) + constants.ONE_DAY_IN_SECONDS;
        clock.setTimeStamp(svm, tommorrow);

        // shrinath fails on second day
        await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: false,
                days_not_synced: 0,
                synced_today: false,
            }
        });

        let shrinathSolBalanceAfterFailDay = Number(svm.getBalance(shrinath.publicKey));
        let shrianthLockedBalanceAfterFailDay = Number((await state.fetchUserPda(svm, programId, shrinath.publicKey)).locked_balance);

        assert(shrianthLockedBalanceAfterFailDay == shrianthLockedBalanceBeforeFailDay - (shrianthLockedBalanceBeforeFailDay * 0.25), "expected 25% of locked balance to be slashed after failing challenge");

        assert(shrinathSolBalanceAfterFailDay == shrinathSolBalanceBeforeFailDay - (constants.DAILY_LAMPORTS + constants.txFeeInLamports), "expected daily lamport to be taken and not recorded in locked balance");

        assert((await state.fetchUserPda(svm, programId, shrinath.publicKey)).streak == 0, "Users streak must be reset after failing challenge on any day");
    });

    it("should slash 25% of locked balance plus daily lamports for yesterday’s miss, but still lock today’s lamport for passing today and keep streak at 1 instead of resetting to 0", async () => {
        let challengeId = 1;

        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);

        // time travel to first day of challenge 
        let challengeStartingTime = Number((await state.fetchChallengePda(svm, programId, challengeId)).start);
        await clock.setTimeStamp(svm, challengeStartingTime);

        // shrinath successfuly passes and locks on first day
        await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: true,
                days_not_synced: 0,
                synced_today: false,
            }
        });

        // pre state
        let shrinathSolBalanceBefore = Number(svm.getBalance(shrinath.publicKey));
        let shrianthLockedBalanceBefore = Number((await state.fetchUserPda(svm, programId, shrinath.publicKey)).locked_balance);

        /// Assume :  shrianth misses syncing on 2nd day

        // time travel to third day
        let thirdDayOfChallenge = challengeStartingTime + (constants.ONE_DAY_IN_SECONDS * 2);
        clock.setTimeStamp(svm, thirdDayOfChallenge);

        // shrianth sync on third day while passing challnge for today
        await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: true,
                days_not_synced: 1,
                synced_today: false,
            }
        });


        // post state
        let shrinathSolBalanceAfter = Number(svm.getBalance(shrinath.publicKey));
        let shrianthLockedBalanceAfter = Number((await state.fetchUserPda(svm, programId, shrinath.publicKey)).locked_balance);

        //// assertions
        assert(shrianthLockedBalanceAfter == shrianthLockedBalanceBefore - (shrianthLockedBalanceBefore * 0.25) + constants.DAILY_LAMPORTS, "expected 25% of locked balance to be slashed and then incremented with todays locked balance since user passed the challenge today");

        assert(shrinathSolBalanceAfter == shrinathSolBalanceBefore - ((constants.DAILY_LAMPORTS * 2) + constants.txFeeInLamports), "expected 2 days of daily lamport to be taken from user, including txfee");

        assert((await state.fetchUserPda(svm, programId, shrinath.publicKey)).streak == 1, "Users streak must be exactly 1 because previous ones are reset while today one is counted");


    });

    it("should reject sync attempts before challenge start, after challenge end, or if user already synced today", async () => {

        let challengeId = 1;

        await userflows.adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);

        // challenge not started 
        let syncBeforeChallengeStartTx = await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: false,
                days_not_synced: 0,
                synced_today: false,
            }
        });

        // time travel to challeng start
        let challengeStartingTime = Number((await state.fetchChallengePda(svm, programId, challengeId)).start);
        await clock.setTimeStamp(svm, challengeStartingTime);

        // user already synced
        let syncAfterAlreadySyncedTx = await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: true,
                days_not_synced: 0,
                synced_today: true,
            }
        });

        // time travel to challeng end
        let challengeEndTime = Number((await state.fetchChallengePda(svm, programId, challengeId)).end);
        await clock.setTimeStamp(svm, challengeEndTime);

        // travel to challenge end 
        let syncAfterChallengeEndTx = await methods.syncAndLock(svm, programId, challengeId, shrinath, {
            logTxResult: false, debug: {
                user_passed: true,
                days_not_synced: 1,
                synced_today: false,
            }
        });

        //// assertions
        assert(txCheck.isFailedTransaction(syncBeforeChallengeStartTx), "user must not be able to sync before the challenge starts");
        assert(txCheck.isFailedTransaction(syncAfterAlreadySyncedTx), "user must not be able to sync more than once in the same day");
        assert(txCheck.isFailedTransaction(syncAfterChallengeEndTx), "user must not be able to sync after the challenge has ended");

    });

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

    it("should prevent non-enrolled users from claiming the winner position of a challenge", async () => {
        let firstChallengeId = 1;
        let secondChallengeId = 2;

        // challenge id == 1,  is in contention phase , berg has more streak than shrinath
        await userflows.teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, firstChallengeId);

        // challenge id == 2, is still active with apaar having positive streak and locked balance
        await userflows.secondChallengeCreatedByBerg____JoinedAndLockedByApaar(svm, programId, berg, apaar, secondChallengeId);


        // apaar tries to claim winner position of the challenge in which she is not enrolled in
        let nonEnrolledWinnerClaimTx = await methods.claimWinnerPosition(svm, programId, firstChallengeId, apaar, { logTxResult: false });

        //// assertion
        assert(txCheck.isFailedTransaction(nonEnrolledWinnerClaimTx), "non-enrolled users must not be able to claim the winner position");

    });

    it("should correctly decrease treasury and user balances and close the UserPDA on withdrawal", async () => {
        let challengeId = 1;


        // teleport to reward claiming phase
        await userflows.teleportToRewardClaimingPhase_________bergDeclaredWinner(svm, programId, admin, jeff, shrinath, berg, challengeId);

        //// pre withdraw state
        let txFeeInLamports = 5_000;
        let [bergUserPDA] = await state.getUserPDAAddressAndBump(berg.publicKey, programId);
        let bergLockedBalance = Number((await state.fetchUserPda(svm, programId, berg.publicKey)).locked_balance);
        let bergSOLBalanceBefore = Number(svm.getBalance(berg.publicKey));
        let treasurySOLBalanceBefore = Number(svm.getBalance(globalPDA));
        let bergUserPDARentBefore = Number(svm.getBalance(bergUserPDA));


        // withdraw
        await methods.withdrawAndClose(svm, programId, challengeId, berg, { logTxResult: false });


        // post withdraw state
        let bergUserPDARentAfter = Number(svm.getBalance(bergUserPDA));
        let bergSOLBalanceAfter = Number(svm.getBalance(berg.publicKey));
        let treasurySOLBalanceAfter = Number(svm.getBalance(globalPDA));

        //// assertions
        assert(bergUserPDARentAfter == 0, "UserPDA rent must be zero after withdrawal, indicating the PDA has been closed.");
        assert(treasurySOLBalanceAfter == treasurySOLBalanceBefore - bergLockedBalance, "Treasury balance should decrease by the withdrawing user's locked balance");
        assert(bergSOLBalanceAfter == (bergSOLBalanceBefore + bergLockedBalance + bergUserPDARentBefore) - txFeeInLamports, "Berg's SOL balance should reflect the sum of his locked balance and rent from the closed PDA, minus the transaction fee.");
    });

    it("should prevent users from withdrawing locked balance from an active challenge by exploiting another challenge’s reward phase ", async () => {
        let firstChallengeId = 1;
        let secondChallengeId = 2;

        // challenge id == 1,  is in contention phase , berg has more streak than shrinath
        await userflows.teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, firstChallengeId);

        // challenge id == 2, is still active with apaar having positive streak and locked balance
        await userflows.secondChallengeCreatedByBerg____JoinedAndLockedByApaar(svm, programId, berg, apaar, secondChallengeId);

        // time travel to challenge Id 1's reward claiming phase
        let now = await clock.now(svm);
        let firstChallengeEnd = Number((await state.fetchChallengePda(svm, programId, firstChallengeId)).end);
        let firstChallengeRewardClaimingPhase = firstChallengeEnd + (constants.ONE_DAY_IN_SECONDS * 5) + 1;
        await clock.setTimeStamp(svm, firstChallengeRewardClaimingPhase);

        // apaar tries to withdraw in challenge id 1's withdraw timeline, in which he is not enrolled in
        let nonEnrolledUserWithdrawTx = await methods.withdrawAndClose(svm, programId, firstChallengeId, apaar, { logTxResult: false });

        //// assertion
        assert(txCheck.isFailedTransaction(nonEnrolledUserWithdrawTx), "users must not be able to withdraw from an active challenge by piggybacking on another challenge’s end timeline");
    });


    it("should prevent withdrawals and reward claiming before the contention period ends", async () => {
        let challengeId = 1;

        // teleport to reward claiming phase
        await userflows.teleportToRewardClaimingPhase_________bergDeclaredWinner(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // time travel back to contention period
        let contentionPeriod = (await clock.now(svm)) - 1;
        await clock.setTimeStamp(svm, contentionPeriod);

        // berg tries to withdraw
        let bergWithdrawalTx = await methods.withdrawAndClose(svm, programId, challengeId, berg, { logTxResult: false });

        // berg tries to claim winner rewards
        let bergWinnerRewardClaimingTx = await methods.claimRewardsAsWinner(svm, programId, challengeId, berg, { logTxResult: false });

        // jeff tries to claim creator rewards
        let jeffCreatorRewardClaimingTx = await methods.claimRewardsAsCreator(svm, programId, challengeId, jeff, { logTxResult: false });


        assert(txCheck.isFailedTransaction(bergWithdrawalTx), "withdrawal should fail if attempted before the contention period ends");
        assert(txCheck.isFailedTransaction(bergWinnerRewardClaimingTx), "winner should not be able to claim rewards before contention period is over");
        assert(txCheck.isFailedTransaction(jeffCreatorRewardClaimingTx), "creator should not be able to claim rewards before contention period is over");
    });

    it("should allow only the winner and challenge creator to claim their respective rewards, and only once", async () => {
        let challengeId = 1;

        // teleport to reward claiming phase
        await userflows.teleportToRewardClaimingPhase_________bergDeclaredWinner(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // winner reward claims
        let bergFirstClaim = await methods.claimRewardsAsWinner(svm, programId, challengeId, berg, { logTxResult: false });

        let bergSecondClaim = await methods.claimRewardsAsWinner(svm, programId, challengeId, berg, { logTxResult: false }); // attempt second claim

        // creator reward claims
        let jeffFirstClaim = await methods.claimRewardsAsCreator(svm, programId, challengeId, jeff, { logTxResult: false });

        let jeffSecondClaim = await methods.claimRewardsAsCreator(svm, programId, challengeId, jeff, { logTxResult: false }); // attempt second claim

        // unauthorized claims
        let shrinathClaim = await methods.claimRewardsAsWinner(svm, programId, challengeId, shrinath, { logTxResult: false });
        let adminClaim = await methods.claimRewardsAsCreator(svm, programId, challengeId, admin, { logTxResult: false });


        //// assertions
        assert(txCheck.isSuccessfulTransaction(bergFirstClaim), "first winner reward claim should succeed for Berg");
        assert(txCheck.isSuccessfulTransaction(jeffFirstClaim), "first creator reward claim should succeed for Jeff");

        assert(txCheck.isFailedTransaction(bergSecondClaim), "second winner reward claim should fail for Berg");
        assert(txCheck.isFailedTransaction(jeffSecondClaim), "second creator reward claim should fail for Jeff");

        assert(txCheck.isFailedTransaction(shrinathClaim), "unauthorized winner reward claim should fail for Shrinath");
        assert(txCheck.isFailedTransaction(adminClaim), "unauthorized creator reward claim should fail for Admin");
    });

    it("should correctly update state, treasury profits, distribute winner and creator rewards, and close the challenge account", async () => {
        let challengeId = 1;
        const [challengePDA] = await state.getChallengePDAAddressAndBump(challengeId, programId);


        // teleport to reward claiming phase
        await userflows.teleportToRewardClaimingPhase_________bergDeclaredWinner(svm, programId, admin, jeff, shrinath, berg, challengeId);

        // pre state
        let txFeeInLamports = 5_000;
        let totalSlashed = Number((await state.fetchChallengePda(svm, programId, challengeId)).total_slashed);
        let Percent50OfTotalSlashed = 0.5 * totalSlashed;
        let Percent40OfTotalSlashed = 0.4 * totalSlashed;
        let Percent10OfTotalSlashed = 0.1 * totalSlashed;

        let creatorSolBalanceBefore = Number(svm.getBalance(jeff.publicKey));
        let winnerSolBalanceBefore = Number(svm.getBalance(berg.publicKey));
        let treasuryProfitsBefore = Number((await state.fetchGlobalPda(svm, programId)).treasury_profits);
        let challengePDARentBefore = Number(svm.getBalance(challengePDA));

        // creator claim
        await methods.claimRewardsAsCreator(svm, programId, challengeId, jeff, { logTxResult: false });

        // winner claim :: note he will get rent from closing challenge account, since the last one to claim among creator and winner gets to keep the rent
        await methods.claimRewardsAsWinner(svm, programId, challengeId, berg, { logTxResult: false });

        //// post state
        let creatorSolBalanceAfter = Number(svm.getBalance(jeff.publicKey));
        let winnerSolBalanceAfter = Number(svm.getBalance(berg.publicKey));
        let treasuryProfitsAfter = Number((await state.fetchGlobalPda(svm, programId)).treasury_profits);
        let challengePDAData = await state.fetchChallengePda(svm, programId, 1, { consoleLog: false });
        let challengePDARentAfter = Number(svm.getBalance(challengePDA));



        //// assertions
        assert(challengePDAData.winner == constants.defaultPubkey.toBase58(), "winner should reset to default pubkey after claiming to prevent multiple claims");
        assert(challengePDAData.creator == constants.defaultPubkey.toBase58(), "creator should reset to default pubkey after claiming to prevent multiple claims");
        assert(challengePDAData.winner_has_claimed, "winner_has_claimed flag should be true after claiming");
        assert(challengePDAData.creator_has_claimed, "creator_has_claimed flag should be true after claiming");

        assert(treasuryProfitsAfter == treasuryProfitsBefore + Percent40OfTotalSlashed, "treasury profits should increase by 40% of total slashed after reward claims");
        assert(creatorSolBalanceAfter == (creatorSolBalanceBefore + Percent10OfTotalSlashed) - txFeeInLamports, "creator should receive 10% of total slashed as rewards");

        assert(winnerSolBalanceAfter == (winnerSolBalanceBefore + Percent50OfTotalSlashed + challengePDARentBefore) - txFeeInLamports, "winner should receive 50% of total slashed plus challenge PDA rent as rewards (since he closed challenge account by claiming after creator)");

        assert(challengePDARentAfter == 0, "challenge account must be fully drained to be recognized as closed by the runtime");
    });






    //@main__important ::  take protocol_protis ---? admin cant drain more than that ??? + need another challenge on going where some slashed .....UUU above one setup similar ??/


});