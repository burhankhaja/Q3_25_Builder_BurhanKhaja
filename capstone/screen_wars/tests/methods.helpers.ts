import { LiteSVM } from "litesvm";

import {
    Keypair,
    LAMPORTS_PER_SOL,
    PublicKey,
    Transaction,
    TransactionInstruction,
    SystemProgram
} from "@solana/web3.js";
import { Buffer } from 'node:buffer';
import { BN } from "bn.js";

//////
// import { LiteSVM, Clock, TransactionMetadata, FailedTransactionMetadata } from "litesvm";
// import * as borsh from "@coral-xyz/borsh";
// import { BN } from "bn.js";
// import { assert, timeStamp } from "node:console";
//////


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