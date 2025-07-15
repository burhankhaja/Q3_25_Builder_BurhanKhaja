import * as anchor from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import BN from "bn.js";

export async function getUserVaultWithBump(userPubkeyBuffer: Buffer, programId: anchor.web3.PublicKey) {

    let [userVaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
          [Buffer.from("vault"), userPubkeyBuffer],
          programId
        );

    return [userVaultPda, vaultBump]; 
}


export async function initializeVault(program: anchor.Program<Vault>)  {
    await program.methods.initVaultAccount().rpc(); //@footnote 1
}

export async function deposit(program: anchor.Program<Vault>, amount: BN){
     await program.methods.deposit(amount).rpc(); //@dev-mistake :: holy-shit :: i had forgot to await :: which caused weird confusions
  
}

export async function withdraw(program: anchor.Program<Vault>, amount: BN){
     await program.methods.withdraw(amount).rpc();
  
}

export async function closeVault(program: anchor.Program<Vault>){
     await program.methods.closeAccount().rpc();
}



// TEMPLATE : export async function _NAME_(program: anchor.Program<Vault>) {}


/*
       // FOOTNOTES


- footnote 1
since accounts are derived within achor, no need to pass them redundantly here, always pass only accounts that the program itself isn't deriving
```
    // Call initialize
    // await program.methods
    //   .initVaultAccount()
    //   .accounts({
    //     signer: provider.wallet.publicKey,
    //     userVault: userVaultPda,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //   })
    //   .rpc(); 
```

*/