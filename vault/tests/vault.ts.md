import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Vault } from "../target/types/vault";
import BN from "bn.js";





describe("UserVault initialize", () => {
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);
  let connection  = provider.connection;

  const program = anchor.workspace.Vault as Program<Vault>;

  let userVaultPda: anchor.web3.PublicKey;
  let vaultBump: number;


  it("Initializes the UserVault", async () => {
    // get PDA and bump of user vault
    // const  [userVaultPda, vaultBump] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);

    const  [userVaultPda, vaultBump] = await getUserVaultWithBump(provider.wallet.publicKey.toBuffer(), program.programId);


    // await program.methods.initVaultAccount().rpc(); 
    // await initializeVault(program); // signer provided just for the sake of different context switching
    await program.methods.initVaultAccount().rpc();


    // Fetch the account
    // const userVault = await program.account.userVault.fetch(userVaultPda);

    //@audit:: MUST :: Check if vault_bump was stored correctly :: use expect , assert from chai/mocha

    // logs
    // console.log("UserVault fetched successfully:", userVault);
    console.log("direct vault bump: ", vaultBump)
    console.log(userVaultPda); 

    // deposit

    let amount: BN = new BN(3000000000);
    await program.methods.deposit(amount).rpc();







    //consoles
     const userVaultBalance = await provider.connection.getBalance(userVaultPda);
       console.log("user vault balance : ", userVaultBalance);


  });


  /////////////////////////////////////////////////////////////////////////////////////////////////////////////////

  //  it("deposit", async () => {

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







});

 async function getUserVaultWithBump(userPubkeyBuffer: Buffer, programId: anchor.web3.PublicKey) {

    let [userVaultPda, vaultBump] = await anchor.web3.PublicKey.findProgramAddress(
          [Buffer.from("vault"), userPubkeyBuffer],
          programId
        );

    return [userVaultPda, vaultBump]; 
}

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