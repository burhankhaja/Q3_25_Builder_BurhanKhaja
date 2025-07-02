use solana_client::rpc_client::RpcClient;
// use solana_sdk::hash::hash; // join it later in below import
use solana_sdk::{
    commitment_config::CommitmentConfig,
    hash::hash,
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey as SdkPubkey,
    signature::{Keypair, Signer, read_keypair_file},
    system_program,
    transaction::Transaction,
};

use solana_program::{pubkey::Pubkey as ProgramPubkey, system_instruction::transfer};

use bs58;
use std::io::{self, BufRead};
use std::str::FromStr;

// const RPC_URL: &str =
//     "https://turbine-solanad-4cde.devnet.rpcpool.com/9a9da9cf-6db1-47dc-839a-55aca5
// c9c80a";

const RPC_URL: &str = "https://api.devnet.solana.com"; // use this directly

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keygen() {
        // Create a new keypair
        let kp = Keypair::new();
        println!(
            "You've generated a new Solana wallet: {}",
            kp.pubkey().to_string()
        );
        println!("");
        println!("To save your wallet, copy and paste the following into a JSON file:");
        println!("{:?}", kp.to_bytes()); // secret key
    }

    #[test]
    fn airdrop() {
        // Import our keypair
        let keypair = read_keypair_file("./dev-wallet.json").expect("Couldn't find wallet file");

        // connection establishment
        // let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());
        let client: RpcClient = RpcClient::new_with_commitment(
            String::from("https://api.devnet.solana.com"),
            CommitmentConfig::confirmed(),
        );

        // request 2 sols
        // match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64).await? {
        match client.request_airdrop(&keypair.pubkey(), 2_000_000_000u64) {
            Ok(sig) => {
                println!("Success! Check your TX here:");
                println!("https://explorer.solana.com/tx/{}?cluster=devnet", sig);
            }

            Err(err) => {
                println!("Airdrop failed: {}", err);
            }
        }

        

        /*Note:: error was in rpc string, so i changed that to https://api.devnet.solana.com and plus i used strict commitment */
    }

    #[test]
    fn transfer_sol() {
        // Load your devnet keypair from file
        let keypair = read_keypair_file("dev-wallet.json").expect("Couldn't find wallet file");

        /*
        // Generate a signature from the keypair
        let pubkey = keypair.pubkey();
        let message_bytes = b"I verify my Solana Keypair!";
        let sig = keypair.sign_message(message_bytes);
        let sig_hashed = hash(sig.as_ref());
        // Verify the signature using the public key
        match sig.verify(&pubkey.to_bytes(), &sig_hashed.to_bytes()) {
            true => println!("Signature verified"),
            false => println!("Verification failed"),
        }
            */

        // Turbin3-wallet public key
        let to_pubkey =
            SdkPubkey::from_str("5gLcyi3EZK56aVc6XEurDiXvk74s9xMHKhFAsAAyZfLg").unwrap();
        let rpc_client = RpcClient::new_with_commitment(
            String::from("https://api.devnet.solana.com"),
            CommitmentConfig::confirmed(),
        );

        let recent_blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");
        /*  // For first transfer of 0.1 sol
        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, 10000000)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");
        println!(
            "Success! Check out your TX here: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
        */

        // getting current balance
        let balance = rpc_client
            .get_balance(&keypair.pubkey())
            .expect("Failed to get balance");

        // building mock tx for fee calculations
        let message = Message::new_with_blockhash(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance)],
            Some(&keypair.pubkey()),
            &recent_blockhash,
        );

        let fee = rpc_client
            .get_fee_for_message(&message)
            .expect("Failed to get fee calculator");

        let transaction = Transaction::new_signed_with_payer(
            &[transfer(&keypair.pubkey(), &to_pubkey, balance - fee)],
            Some(&keypair.pubkey()),
            &vec![&keypair],
            recent_blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send final transaction");

        println!(
            "Success! Entire balance transferred: https://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );
    }

    #[test]
    fn base58_to_wallet() {
        println!("Input your private key as a base58 string:");
        let stdin = io::stdin();
        let base58 = stdin.lock().lines().next().unwrap().unwrap();
        println!("Your wallet file format is:");
        let wallet = bs58::decode(base58).into_vec().unwrap();
        println!("{:?}", wallet);
    }
   

    #[test]
    fn wallet_to_base58() {
        println!("Input your private key as a JSON byte array (e.g. [12,34,...]):");
        let stdin = io::stdin();
        let wallet = stdin
            .lock()
            .lines()
            .next()
            .unwrap()
            .unwrap()
            .trim_start_matches('[')
            .trim_end_matches(']')
            .split(',')
            .map(|s| s.trim().parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        println!("Your Base58-encoded private key is:");
        let base58 = bs58::encode(wallet).into_string();
        println!("{:?}", base58);
    }

    // @note :: Main Function of this Pre-req
    #[test]
    fn execute_submit_rs() {
        let signer = read_keypair_file("./Turbin3-wallet.json").expect("Couldn't find wallet file");

        // quick debug
        println!("Signer public key: {}", signer.pubkey());

        let rpc_client = RpcClient::new_with_commitment(
            String::from("https://api.devnet.solana.com"),
            CommitmentConfig::confirmed(),
        );

        // Mint account for my collection id
        let mint = Keypair::new();
        println!("Mint Secret key (bytes): {:?}", mint.to_bytes());
        println!("Mint public key: {}", mint.pubkey());

        let turbin3_prereq_program =
            ProgramPubkey::from_str("TRBZyQHB3m68FGeVsqTK39Wm4xejadjVhP5MAZaKWDM").unwrap();

        let collection =
            ProgramPubkey::from_str("5ebsp5RChCGK7ssRZMVMufgVZhd2kFbNaotcZ5UvytN2").unwrap();

        let mpl_core_program =
            ProgramPubkey::from_str("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d").unwrap();

        let system_program = system_program::id();

        let signer_pubkey = signer.pubkey();

        let my_pda_seeds = &[b"prereqs", signer_pubkey.as_ref()];
        let (prereq_pda, _bump) =
            SdkPubkey::find_program_address(my_pda_seeds, &turbin3_prereq_program);

        let (authority_pda, _) = SdkPubkey::find_program_address(
            &[b"collection", collection.as_ref()],
            &turbin3_prereq_program,
        );

        // Debug
        // println!("PDA: {}", prereq_pda); // correct PDA
        println!("Authority PDA: {}", authority_pda); // correct authority PDA

        let data = vec![77, 124, 82, 163, 21, 133, 181, 206];

        let accounts = vec![
            AccountMeta::new(signer.pubkey(), true), // user signer
            AccountMeta::new(prereq_pda, false),     // PDA account
            AccountMeta::new(mint.pubkey(), true),   // mint keypair
            AccountMeta::new(collection, false),     // collection
            AccountMeta::new_readonly(authority_pda, false), // authority (PDA) 
            AccountMeta::new_readonly(mpl_core_program, false), // mpl core program
            AccountMeta::new_readonly(system_program, false), // system program
        ];

        let blockhash = rpc_client
            .get_latest_blockhash()
            .expect("Failed to get recent blockhash");

        let instruction = Instruction {
            program_id: turbin3_prereq_program,
            accounts,
            data,
        };

        let transaction = Transaction::new_signed_with_payer(
            &[instruction],
            Some(&signer.pubkey()),
            &[&signer, &mint],
            blockhash,
        );

        let signature = rpc_client
            .send_and_confirm_transaction(&transaction)
            .expect("Failed to send transaction");

        println!(
            "Success! Check out your TX here:\nhttps://explorer.solana.com/tx/{}/?cluster=devnet",
            signature
        );

        
    }
}

/*
{
      "name": "submit_rs",
      "discriminator": [
        77,
        124,
        82,
        163,
        21,
        133,
        181,
        206
      ],
      "accounts": [
        {
          "name": "user",
          "writable": true,
          "signer": true
        },
        {
          "name": "account",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  112,
                  114,
                  101,
                  114,
                  101,
                  113,
                  115
                ]
              },
              {
                "kind": "account",
                "path": "user"
              }
            ]
          }
        },
        {
          "name": "mint",
          "writable": true,
          "signer": true
        },
        {
          "name": "collection",
          "writable": true
        },
        {
          "name": "authority",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  99,
                  111,
                  108,
                  108,
                  101,
                  99,
                  116,
                  105,
                  111,
                  110
                ]
              },
              {
                "kind": "account",
                "path": "collection"
              }
            ]
          }
        },
        {
          "name": "mpl_core_program",
          "address": "CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": []
    },*/
