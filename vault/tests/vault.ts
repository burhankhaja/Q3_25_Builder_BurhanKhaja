import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import BN from "bn.js";

//helpers
import {getUserVaultWithBump, initializeVault, deposit, withdraw, closeVault} from "./helpers";

// describe("vault", () => {
//  const provider = anchor.AnchorProvider.env();
//   anchor.setProvider(provider);


//   const program = anchor.workspace.vault as Program<Vault>;

//   it("Is initialized!", async () => {
//     // Add your test here.
//     const tx = await program.methods.initVaultAccount().rpc();
//     // console.log("Your transaction signature", tx);
//     // console.log("wallet: ", provider.wallet.publicKey.toBase58());



//   });
// });

describe("UserVault initialize", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let connection  = provider.connection;

  const program = anchor.workspace.Vault as Program<Vault>;

  let userVaultPda: anchor.web3.PublicKey;
  let vaultBump: number;

  // before(async() => { //@audit ::: update logic such that new signers are spawned on each test
  //   try {
  //     await closeVault(program);
  //   } catch(err) {}
  // })




  // it("Initializes the UserVault", async () => {
  //   // get PDA and bump of user vault
  //   // const  [userVaultPda, vaultBump] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);

  //   const  [userVaultPda, vaultBump] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);


  //   // await program.methods.initVaultAccount().rpc(); 
  //   await initializeVault(program); // signer provided just for the sake of different context switching


  //   // Fetch the account
  //   const userVault = await program.account.userVault.fetch(userVaultPda);

  //   //@audit:: MUST :: Check if vault_bump was stored correctly :: use expect , assert from chai/mocha

  //   // logs
  //   console.log("UserVault fetched successfully:", userVault);
  //   console.log("direct vault bump: ", vaultBump)
  //   console.log(userVaultPda); 

  // });


  // /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

  //  it("deposit test", async () => { //@audit- fails

  //      await initializeVault(program);


  //      let amount: BN = new BN(1000000000);
  //      const  [userVault] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);
  //      const before = await provider.connection.getBalance(provider.wallet.publicKey);

  //      const depositSig = await deposit(program, amount);
  //      connection.confirmTransaction(depositSig, "confirmed");
       

  //      const after = await provider.connection.getBalance(provider.wallet.publicKey);


  //      const userVaultBalance = await provider.connection.getBalance(userVault);
  //      console.log("user vault balance : ", userVaultBalance);
  //      console.log("before:", before);
  //      console.log("after:", after);


  //  });


  //@audit-ok ::: worked fine ... even the unorthodox one

   it("withdraw test", async() => { //@audit-ok ::: withdraw worked !!!!
      let sol = 1_000_000_000;
      let depositAmount = new BN(2*sol);
      let withdrawAmount = new BN(1*sol);
      
      // initialize and deposit 2 sols in user vault
      await initializeVault(program);
      await deposit(program, depositAmount);

      // cache user vault balance before withdraw
      const  [userVault] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);
      const before = await provider.connection.getBalance(userVault);

      // // withdraw 1 sol from user vault
      await withdraw(program, withdrawAmount);  

      // // cache user vault balance after withdraw
      const after = await provider.connection.getBalance(userVault);

      console.log(`vault balance before withdraw: ${before}`)
      console.log(`vault balance after withdraw: ${after}`)


   });


  // // /////////////////////////////////////////////////////////////////////////////////////////////////////////////////
  // // // test initialize-close

  //  it("init close", async()=> { 
 
  //     await initializeVault(program);


  //     const  [userVault] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);
  //     const before = await provider.connection.getBalance(userVault);
  //     const signerBeforeBal = await provider.connection.getBalance(provider.wallet.publicKey);

  //     // close the vault 
  //     await closeVault(program);

  //     const after = await provider.connection.getBalance(userVault);
  //     const signerAfterBal = await provider.connection.getBalance(provider.wallet.publicKey);


  //     console.log(`beforeVaultBal: ${before}`)
  //     console.log(`afterVaultBal: ${after}`) 

  //     console.log(`beforeSignerBal: ${signerBeforeBal}`)
  //     console.log(`afterSignerBal: ${signerAfterBal}`) 



  //  });


  //  // init deposit close
  //  it("init deposit close", async()=> { 
  //     let sol = 1_000_000_000;
  //     let depositAmount = new BN(2*sol);
      
  //     // initialize and deposit 2 sols in user vault
  //     await initializeVault(program);
  //     await deposit(program, depositAmount);
 



  //     const  [userVault] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);
  //     const before = await provider.connection.getBalance(userVault);
  //     const signerBeforeBal = await provider.connection.getBalance(provider.wallet.publicKey);

  //     // close the vault 
  //     await closeVault(program);

  //     const after = await provider.connection.getBalance(userVault);
  //     const signerAfterBal = await provider.connection.getBalance(provider.wallet.publicKey);


  //     console.log(`beforeVaultBal: ${before}`)
  //     console.log(`afterVaultBal: ${after}`) 

  //     console.log(`beforeSignerBal: ${signerBeforeBal}`)
  //     console.log(`afterSignerBal: ${signerAfterBal}`) 



  //  });


  /////////////////////////////////////////////////////////////////////////////////////////////////////////////////


//  it("changing provider in test", async () => {
//   // Generate a new keypair
//   const newKeypair = anchor.web3.Keypair.generate();

//   // Use same connection as current provider
//   const connection = provider.connection;

//   // Create a custom wallet interface from keypair
//   const newWallet = new anchor.Wallet(newKeypair);

//   // Create a new provider with the generated keypair
//   const customProvider = new anchor.AnchorProvider(connection, newWallet, {
//     preflightCommitment: "processed",
//   });

//   // Set this new provider globally
//   anchor.setProvider(customProvider);
//   provider = anchor.getProvider();

//   // get the provider.wallet
//   console.log(`The anchor.provider Is >>>>>>>>>> ${await anchor.getProvider().wallet.publicKey}`)


//   // You can also airdrop to test
//   // await connection.requestAirdrop(customProvider.wallet.publicKey, 1e9);
  
//    //@audit-issue : doesn't increase balance ::
//   // cause we didn't wait for confirmation.... i think that is also the reason why our tx didn't deposit anything >> if that is the case make sure to confirm bug type in devnet.... if true then you always need to validate in you program to check balance increases otherwise can get rigged>>>>>


//   const sig =  await connection.requestAirdrop(customProvider.wallet.publicKey, 1e9);
//   await connection.confirmTransaction(sig, "confirmed");
  

//   // Check its balance (will be 0 SOL unless you airdrop)
//   const balance = await connection.getBalance(anchor.getProvider().wallet.publicKey);
//   console.log("New wallet SOL balance:", balance);




//   // call deposit




//      await initializeVault(program);


//        let amount: BN = new BN(1000000000);
//        const  [userVault] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);
//        const before = await provider.connection.getBalance(provider.wallet.publicKey);

//        const depositSig = await deposit(program, amount);
//        connection.confirmTransaction(depositSig, "confirmed");
       

//        const after = await provider.connection.getBalance(provider.wallet.publicKey);


//        const userVaultBalance = await provider.connection.getBalance(userVault);
//        console.log("user vault balance : ", userVaultBalance);
//        console.log("before:", before);
//        console.log("after:", after);

//      const userVaultFetch = await program.account.userVault.fetch(userVault);
//          console.log("user vault pda:", userVaultFetch);

// });


// it("bug", {
  // await connection.requestAirdrop(customProvider.wallet.publicKey, 1e9); //@audit-issue : doesn't increase balance ::
  // cause we didn't wait for confirmation.... i think that is also the reason why our tx didn't deposit anything >> if that is the case make sure to confirm bug type in devnet.... if true then you always need to validate in you program to check balance increases otherwise can get rigged>>>>>
// })




});


// TEMPLATE ::  it("", async () => {});

/*
       // FOOTNOTES






     // SOME HELPER NOTES TO TAKE  

          // it ("some helper tests: ", async() => {
      //  const balance= await provider.connection.getBalance(provider.wallet.publicKey);
      //  console.log("balance of provider.wallet: ", balance)

      //  const bob = anchor.web3.Keypair.generate();
      //  const balanceArb = await provider.connection.getBalance(bob.publicKey);
      //  console.log("bobs balance", balanceArb)

      //  // conclusion :: 
      //  // new keypairs have 0 sol balance
      //  // provider.wallet has huge sol balances

      //  // use anchor.web3 instead of importing from solana/web3
      // });

*/