import { LiteSVM } from "litesvm";
import {
    Keypair,
    PublicKey,
} from "@solana/web3.js";

// helper imports
import * as methods from "./methods.helpers";
import * as state from "./state.helpers";
import * as clock from "./clock.helpers";
import * as constants from "./constants.helpers";

export async function adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm: LiteSVM, programId: PublicKey, admin: Keypair, jeff: Keypair, shrinath: Keypair, berg: Keypair) {
    
    //// admin initializes
    let [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);
    await methods.initialize(svm, programId, admin, globalPDA);

    //// jeff create challenge (id: 1)
    let startTime = await clock.now(svm) + constants.ONE_DAY_IN_SECONDS;
    let dailyTimer = constants.ONE_HOUR_IN_SECONDS;
    let globalPDAData = await state.fetchGlobalPda(svm, programId);
    const [challengePDA] = await state.getChallengePDAAddressAndBump(globalPDAData.challenge_ids, programId);
    await methods.createChallenge(svm, programId, startTime, dailyTimer, jeff, globalPDA, challengePDA, { logTxResult: false });
    let initialChallengePDAData = await state.fetchChallengePda(svm, programId, globalPDAData.challenge_ids, { consoleLog: false });

    //// shrinath joins challenge (id: 1)
    let _challengeId = initialChallengePDAData.challenge_id;
    let [shrinathUserPDA] = await state.getUserPDAAddressAndBump(shrinath.publicKey, programId);
    await methods.joinChallenge(svm, programId, _challengeId, shrinath, shrinathUserPDA, globalPDA, challengePDA, { logTxResult: false });

    //// Berg also  joins challenge (id: 1)
    let [bergUserPDA] = await state.getUserPDAAddressAndBump(berg.publicKey, programId);


    await methods.joinChallenge(svm, programId, _challengeId, berg, bergUserPDA, globalPDA, challengePDA, { logTxResult: false });
}


export async function challengeRun___shrinathSlashedSomeDays__bergMaxStreak(svm: LiteSVM, programId: PublicKey,shrinath: Keypair, berg: Keypair, challengeId : number) {


    //// @dev :: later :: use svm.setAccount on users pda to mock 19th days of perfect user state ... with both berg and shrinath having 18 streaks and locked_balance == 18 * 10mil lamports
    
    //// timetravel to 19th day of challenge
    let challengePDAData = await state.fetchChallengePda(svm, programId, challengeId, {consoleLog: false});
    let challengeStartTime = challengePDAData.start;


    let day19 = Number(challengeStartTime) + (constants.TWENTY_ONE_DAY_IN_SECONDS - (constants.ONE_DAY_IN_SECONDS * 2)); 

    await clock.setTimeStamp(svm, day19);

    // berg passes on 19th day too
    await methods.syncAndLock(svm, programId, challengeId, berg, { logTxResult: false });
    
    // shrinath fails on 19th day {if mocked prev state =>  slashed + streak_reset}
    await methods.syncAndLock(svm, programId, challengeId, shrinath, { logTxResult: false, debug: {
        user_passed: false,
        days_not_synced: 0,
        synced_today: false,
    } });


    //// time travel to 20th day , last day of challenge
    let day20 = day19 + constants.ONE_DAY_IN_SECONDS;
    await clock.setTimeStamp(svm, day20);


    // berg just keeps passing 
    await methods.syncAndLock(svm, programId, challengeId, berg, { logTxResult: false, debug: {
        user_passed: true,
        days_not_synced: 0,
        synced_today: false,
    } }); //@audit :: weird problem if used default syntax again here :: refer to BACKUP_TESS/IMPOSSIBLE_SYNCLOCK_ERROR.TS.md on local repo

    // shrinath passes on last day gets his positive streak back {streak: 1}
    await methods.syncAndLock(svm, programId, challengeId, shrinath, { logTxResult: false });

    // Time travel to Contention Period (one second after the challenge End ) where users with higher streaks compete to claim winner position
    let challengeEndTime = Number(challengePDAData.end);
    await clock.setTimeStamp(svm, challengeEndTime + 1);
}


export async function teleportToChallengeEnd_____BergOutstreaksShrinath(svm: LiteSVM, programId: PublicKey,admin: Keypair, jeff: Keypair, shrinath: Keypair, berg: Keypair, challengeId : number) {

    await adminInitializes__jeffCreatesChallenge__Shrinath_And_BergJoins(svm, programId, admin, jeff, shrinath, berg);
    
    await challengeRun___shrinathSlashedSomeDays__bergMaxStreak(svm, programId, shrinath, berg, challengeId);
}

// add teleportToREwardClaiingPHase_________bergDeclaredWinner

export async function teleportToRewardClaimingPhase_________bergDeclaredWinner(svm: LiteSVM, programId: PublicKey,admin: Keypair, jeff: Keypair, shrinath: Keypair, berg: Keypair, challengeId : number) {

    await teleportToChallengeEnd_____BergOutstreaksShrinath(svm, programId, admin, jeff, shrinath, berg, challengeId);

    // berg becomes winner
    await methods.claimWinnerPosition(svm, programId, challengeId, berg, { logTxResult: false });

    // time travel to reward claiming phase
    let afterContentionPeriod = (await clock.now(svm) + (constants.ONE_DAY_IN_SECONDS * 5)); // since contention period spawn 5 days after end , and now is the 
    await clock.setTimeStamp(svm, afterContentionPeriod);
}
