import { LiteSVM, Clock, TransactionMetadata } from "litesvm";
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

// Template :: tests
// it("", async() => {})


const programId = new PublicKey("4jqrWDfeR2RAzSPYNoiVq2dcVrZUrsp3ZWEPHehVwCtW");

describe("screen_wars", () => {

    it("call initialize function", async () => {
        const svm = new LiteSVM();
        svm.addProgramFromFile(programId, "./target/deploy/screen_wars.so");

        const payer = new Keypair();
        svm.airdrop(payer.publicKey, BigInt(LAMPORTS_PER_SOL));

        const [globalPDA] = PublicKey.findProgramAddressSync(
            [Buffer.from("global")],
            programId
        );

        // Anchor's 8-byte discriminator for "initialize"
        const initializeDisc = Buffer.from([175, 175, 109, 31, 13, 152, 155, 237]);

        const ix = new TransactionInstruction({
            keys: [
                { pubkey: payer.publicKey, isSigner: true, isWritable: true }, // admin
                { pubkey: globalPDA, isSigner: false, isWritable: true },      // global
                { pubkey: SystemProgram.programId, isSigner: false, isWritable: false },
            ],
            programId,
            data: initializeDisc
        });

        const tx = new Transaction().add(ix);
        tx.recentBlockhash = svm.latestBlockhash();
        tx.sign(payer);

        let result = svm.sendTransaction(tx);
        console.log("logs: ", result.logs())

        let globalPdaData = await fetchGlobalPda(svm, {consoleLog: false})
        console.log("challenge ids XXX : ", globalPdaData.challenge_ids)

    });

});


async function fetchGlobalPda(svm: LiteSVM, options: { consoleLog?: boolean } = {}) {
      const { consoleLog = false } = options;
    const [globalPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("global")],
        programId
    );

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