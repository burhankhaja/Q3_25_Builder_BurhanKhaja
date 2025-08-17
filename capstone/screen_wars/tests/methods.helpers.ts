import { LiteSVM } from "litesvm";

import {
    Keypair,
    PublicKey,
    Transaction,
    TransactionInstruction,
    SystemProgram
} from "@solana/web3.js";
import { Buffer, constants } from 'node:buffer';
import { BN } from "bn.js";

import * as state from "./state.helpers";
import * as constant from "./constants.helpers";


export async function initialize(svm: LiteSVM, programId: PublicKey, admin: Keypair, globalPDA: PublicKey) {
    // Anchor's 8-byte discriminator for "initialize"
    const initializeDisc = Buffer.from([175, 175, 109, 31, 13, 152, 155, 237]);

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: admin.publicKey, isSigner: true, isWritable: true }, // admin
            { pubkey: globalPDA, isSigner: false, isWritable: true },      // global
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data: initializeDisc
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(admin);

    let result = svm.sendTransaction(tx);
    return result;
}



export async function setChallengeCreationPaused(svm: LiteSVM, programId: PublicKey, admin: Keypair, globalPDA: PublicKey, pause: boolean, options: { logTxResult?: boolean } = {}) {

    const { logTxResult = false } = options;


    const pauseDiscriminator = Buffer.from([53, 252, 16, 113, 135, 93, 182, 207]);
    let _pause = [1, 0, 0, 0];
    let _unpause = [0, 0, 0, 0];

    const data = Buffer.concat([
        pauseDiscriminator,
        Buffer.from(pause ? _pause : _unpause)
    ]);

    // Create instruction
    const ix = new TransactionInstruction({
        keys: [
            { pubkey: admin.publicKey, isSigner: true, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true },
        ],
        programId,
        data
    });

    // Send transaction
    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(admin);
    let result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Debugging Transaction status:", "err" in result ? `${result.err().toString()}: ${result.meta().logs()}` : result.logs());
    }

    return result;

}


export async function createChallenge(svm: LiteSVM, programId: PublicKey, _startTime: any, _dailyTimer: any, creator: Keypair, globalPDA: PublicKey, challengePDA: PublicKey, options: { logTxResult?: boolean } = {}) {
    const { logTxResult = false } = options;


    let discriminator = Buffer.from([170, 244, 47, 1, 1, 15, 173, 239]);
    let startTime = new BN(_startTime).toArrayLike(Buffer, "le", 8); //@dev :: i64s match with ["le", 8]s
    let dailyTimer = new BN(_dailyTimer).toArrayLike(Buffer, "le", 8);

    let data = Buffer.concat([
        discriminator,
        Buffer.from(startTime),
        Buffer.from(dailyTimer)
    ])

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: creator.publicKey, isSigner: true, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    let tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(creator);

    let result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Debugging Transaction status:", "err" in result ? `${result.err().toString()}: ${result.meta().logs()}` : result.logs());
    }

    return result;



}


export async function joinChallenge(svm: LiteSVM, programId: PublicKey, _challengeId: any, user: Keypair, userPDA: PublicKey, globalPDA: PublicKey, challengePDA: PublicKey, options: { logTxResult?: boolean } = {}) {
    const { logTxResult = false } = options;


    let discriminator = Buffer.from([41, 104, 214, 73, 32, 168, 76, 79]);
    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);

    let data = Buffer.concat([
        discriminator,
        Buffer.from(challengeId),
    ])

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: user.publicKey, isSigner: true, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: false },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: userPDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    let tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(user);

    let result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Debugging Transaction status:", "err" in result ? `${result.err().toString()}: ${result.meta().logs()}` : result.logs());
    }

    return result;


}


export async function syncAndLock(
    svm: LiteSVM,
    programId: PublicKey,
    _challengeId: number,
    user: Keypair,
    options: {
        logTxResult?: boolean,
        debug?: {
            user_passed: boolean,
            days_not_synced: number,
            synced_today: boolean
        },
    } = {}
) {
    const { logTxResult = false, debug } = options;

    const discriminator = Buffer.from([126, 228, 221, 117, 135, 234, 35, 250]);

    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);


    // serializing Option<DebugData>
    let debugDataBuffer; //@audit-issue : weird error : when set None and if and only if sync is called two times => causes Error 6 ? while calling once with none works fine
    // @audit-ok : temp fix : always use some data
    if (debug) {
        debugDataBuffer = Buffer.concat([
            Buffer.from([1]), // Some flag (1)
            Buffer.from([debug.user_passed ? 1 : 0]),
            Buffer.from([debug.days_not_synced]),
            Buffer.from([debug.synced_today ? 1 : 0])
        ]);
    } else {
        debugDataBuffer = Buffer.from([0]); // None (0)
    }

    const data = Buffer.concat([
        discriminator,
        challengeId,
        debugDataBuffer
    ]);

    // accounts
    const [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);
    const [challengePDA] = await state.getChallengePDAAddressAndBump(_challengeId, programId);
    const [userPDA] = await state.getUserPDAAddressAndBump(user.publicKey, programId);

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: user.publicKey, isSigner: true, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: userPDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(user);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }


    return result;
}

export async function claimWinnerPosition(
    svm: LiteSVM,
    programId: PublicKey,
    _challengeId: number,
    user: Keypair,
    options: {
        logTxResult?: boolean,
    } = {}
) {
    const { logTxResult = false } = options;
    let discriminator = Buffer.from([115, 143, 40, 222, 237, 184, 243, 235]);

    // params
    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);

    // accounts
    const [challengePDA] = await state.getChallengePDAAddressAndBump(_challengeId, programId);
    const [userPDA] = await state.getUserPDAAddressAndBump(user.publicKey, programId);

    // data
    const data = Buffer.concat([
        discriminator,
        challengeId,
    ]);

    // accounts
    // user : signer
    // challengePDA
    // userPDA
    // systemAccount

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: user.publicKey, isSigner: true, isWritable: true },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: userPDA, isSigner: false, isWritable: false },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(user);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }


    return result;

}

export async function withdrawAndClose(
    svm: LiteSVM,
    programId: PublicKey,
    _challengeId: number,
    user: Keypair,
    options: {
        logTxResult?: boolean,
    } = {}
) {
    const { logTxResult = false } = options;
    let discriminator = Buffer.from([226, 34, 214, 71, 139, 182, 0, 238]);

    // params
    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);

    // accounts
    const [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);
    const [challengePDA] = await state.getChallengePDAAddressAndBump(_challengeId, programId);
    const [userPDA] = await state.getUserPDAAddressAndBump(user.publicKey, programId);

    // data
    const data = Buffer.concat([
        discriminator,
        challengeId,
    ]);

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: user.publicKey, isSigner: true, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true }, //@dev : writable cause lamports sent
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: userPDA, isSigner: false, isWritable: true }, //@dev :: writable because closed
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(user);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }


    return result;

}

export async function claimRewardsAsWinner(
    svm: LiteSVM,
    programId: PublicKey,
    _challengeId: number,
    winner: Keypair,
    options: {
        logTxResult?: boolean,
    } = {}
) {

    const { logTxResult = false } = options;
    let discriminator = Buffer.from([158, 96, 78, 224, 80, 254, 44, 164]);

    // params
    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);

    // accounts
    const [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);
    const [challengePDA] = await state.getChallengePDAAddressAndBump(_challengeId, programId);

    // data
    const data = Buffer.concat([
        discriminator,
        challengeId,
    ]);

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: winner.publicKey, isSigner: true, isWritable: true },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(winner);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }


    return result;
}

export async function claimRewardsAsCreator(
    svm: LiteSVM,
    programId: PublicKey,
    _challengeId: number,
    creator: Keypair,
    options: {
        logTxResult?: boolean,
    } = {}
) {

    const { logTxResult = false } = options;
    let discriminator = Buffer.from([42, 100, 134, 87, 170, 75, 36, 122]);

    // params
    let challengeId = new BN(_challengeId).toArrayLike(Buffer, "le", 4);

    // accounts
    const [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);
    const [challengePDA] = await state.getChallengePDAAddressAndBump(_challengeId, programId);

    // data
    const data = Buffer.concat([
        discriminator,
        challengeId,
    ]);

    const ix = new TransactionInstruction({
        keys: [
            { pubkey: creator.publicKey, isSigner: true, isWritable: true },
            { pubkey: challengePDA, isSigner: false, isWritable: true },
            { pubkey: globalPDA, isSigner: false, isWritable: true },
            { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
        ],
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = svm.latestBlockhash();
    tx.sign(creator);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }


    return result;
}

export async function takeProtocolProfits(
    svm: LiteSVM,
    programId: PublicKey,
    _amount: number,
    admin: Keypair,
    options: {
        logTxResult?: boolean,
        toOptional?: PublicKey
    } = {}
) {
    const { logTxResult = false, toOptional } = options;

    let discriminator = Buffer.from([189, 193, 172, 48, 203, 21, 248, 147]);
    let amount = new BN(_amount).toArrayLike(Buffer, "le", 8);


    
    // data
    const data = Buffer.concat([
        discriminator,
        amount,
    ]);

    //// Accounts
    const [globalPDA] = await state.getGlobalPDAAddressAndBump(programId);

    // keys
    const keys = [
        { pubkey: admin.publicKey, isSigner: true, isWritable: true }, // admin
        { pubkey: globalPDA, isSigner: false, isWritable: true },     // global
    ];

    // if optional account is provided funds are sent to that else to admin
    if (toOptional) {
        keys.push({ pubkey: toOptional, isSigner: false, isWritable: true });
    } else {
        // issue :: fails to represent None with defaultPubkey `11111111111111111111111111111111` represents None
        // svm.airdrop(constant.defaultPubkey, BigInt(1_000_000_000));
        // keys.push({ pubkey: constant.defaultPubkey, isSigner: false, isWritable: true }); // returns TransactionErrorInsufficientFundsForRent
        // @audit :: later :: How to represent Optional Account in Manual Tx bundles
        keys.push({ pubkey: admin.publicKey, isSigner: false, isWritable: true }); //fallback if not solved
    }

    keys.push({ pubkey: SystemProgram.programId, isSigner: false, isWritable: false });


    const ix = new TransactionInstruction({
        keys,
        programId,
        data
    });

    const tx = new Transaction().add(ix);
    tx.recentBlockhash = await svm.latestBlockhash();
    tx.sign(admin);

    const result = svm.sendTransaction(tx);

    if (logTxResult) {
        console.log("Transaction result:",
            "err" in result
                ? `Error: ${result.err().toString()}\nLogs: ${result.meta().logs()}`
                : `Success\nLogs: ${result.logs()}`
        );
    }

    return result;
}