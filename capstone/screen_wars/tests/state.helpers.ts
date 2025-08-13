import { LiteSVM } from "litesvm";

import {
    PublicKey,
} from "@solana/web3.js";
import { Buffer } from 'node:buffer';
import * as borsh from "@coral-xyz/borsh";
import { BN } from "bn.js";


/////////////////////////////////////////////
//// helpers :: log and fetch pda data  ////
///////////////////////////////////////////


// globalPDA
export async function fetchGlobalPda(svm: LiteSVM, programId: PublicKey, options: { consoleLog?: boolean } = {}) {
    const { consoleLog = false } = options;

    const [globalPDA] = await getGlobalPDAAddressAndBump(programId);

    // --- Decode global account ---
    const GLOBAL_LAYOUT = borsh.struct([
        borsh.publicKey("admin"),
        borsh.publicKey("treasury"),
        borsh.u64("treasury_profits"),
        borsh.u32("challenge_ids"),
        borsh.u8("challenge_creation_paused"),
        borsh.u8("bump"),
    ]);

    const acctInfo = svm.getAccount(globalPDA);
    const data = Buffer.from(acctInfo.data).slice(8); // skip Anchor discriminator
    const decoded = GLOBAL_LAYOUT.decode(data);

    if (consoleLog) {
        console.log({
            admin: new PublicKey(decoded.admin).toBase58(),
            treasury: new PublicKey(decoded.treasury).toBase58(),
            treasury_profits: decoded.treasury_profits.toString(),
            challenge_ids: decoded.challenge_ids,
            challenge_creation_paused: decoded.challenge_creation_paused !== 0,
            bump: decoded.bump
        });

    }

    return decoded;

}


// challengePDA
export async function fetchChallengePda(svm: LiteSVM, programId: PublicKey, challengeId: any, options: { consoleLog?: boolean } = {}) {
    const { consoleLog = false } = options;

    const [challengePDA] = await getChallengePDAAddressAndBump(challengeId, programId);

    // --- Decode challenge account ---
    const CHALLENGE_LAYOUT = borsh.struct([
        borsh.publicKey("creator"),
        borsh.u32("challenge_id"),
        borsh.i64("daily_timer"),
        borsh.i64("start"),
        borsh.i64("end"),
        borsh.u64("total_slashed"),
        borsh.publicKey("winner"),
        borsh.u8("winner_streak"),
        borsh.bool("winner_has_claimed"),
        borsh.bool("creator_has_claimed"),
        borsh.u32("total_participants"),
        borsh.u8("bump"),
    ]);

    const acctInfo = svm.getAccount(challengePDA);
    const data = Buffer.from(acctInfo.data).slice(8); // skip Anchor discriminator
    const decoded = CHALLENGE_LAYOUT.decode(data);

    if (consoleLog) {
        console.log({
            creator: new PublicKey(decoded.creator).toBase58(),
            challenge_id: decoded.challenge_id,
            daily_timer: decoded.daily_timer.toString(),
            start: decoded.start.toString(),
            end: decoded.end.toString(),
            total_slashed: decoded.total_slashed.toString(),
            winner: new PublicKey(decoded.winner).toBase58(),
            winner_streak: decoded.winner_streak,
            winner_has_claimed: decoded.winner_has_claimed,
            creator_has_claimed: decoded.creator_has_claimed,
            total_participants: decoded.total_participants,
            bump: decoded.bump
        });
    }

    return decoded;
}


// userPDA
export async function fetchUserPda(svm: LiteSVM, programId: PublicKey, userPubkey : PublicKey, options: { consoleLog?: boolean } = {}) {
    const { consoleLog = false } = options;
    const [userPDA] = await getUserPDAAddressAndBump(userPubkey, programId);

    // --- Decode user account ---
    const USER_LAYOUT = borsh.struct([
        borsh.publicKey("user"),
        borsh.u32("challenge_id"),
        borsh.u64("locked_balance"),
        borsh.u8("streak"),
        borsh.u8("bump"),
    ]);

    const acctInfo = svm.getAccount(userPDA);
    const data = Buffer.from(acctInfo.data).slice(8); // skip Anchor discriminator
    const decoded = USER_LAYOUT.decode(data);

    if (consoleLog) {
        console.log({
            user: new PublicKey(decoded.user).toBase58(),
            challenge_id: decoded.challenge_id,
            locked_balance: decoded.locked_balance.toString(),
            streak: decoded.streak,
            bump: decoded.bump
        });
    }

    return decoded;

}



/////////////////////////////////////////////////
//// helpers : pda addresses and their bumps ///
///////////////////////////////////////////////


export async function getGlobalPDAAddressAndBump(
    programId: PublicKey
): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress([Buffer.from("global")], programId);
}

export async function getChallengePDAAddressAndBump(
    challengeId: any,
    programId: PublicKey,
): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress([Buffer.from("challenge"),  new BN(challengeId).toArrayLike(Buffer, "le", 4)], programId);
}

export async function getUserPDAAddressAndBump(
    userPubkey: PublicKey,
    programId: PublicKey,
): Promise<[PublicKey, number]> {
    return PublicKey.findProgramAddress([Buffer.from("user"), userPubkey.toBuffer()], programId);
}

