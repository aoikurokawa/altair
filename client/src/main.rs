use std::str::FromStr;

use clap::{Parser, Subcommand};
use client::Client;
use solana_sdk::{pubkey::Pubkey, signature::read_keypair_file};
use tokio::{io, runtime::Runtime};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    CopyTransaction {
        copy_program: String,
        account_for_proof: String,
        #[arg(long, required = true)]
        /// Path to the signer key
        signer: String,
        #[arg(short, long)]
        /// URL for solana RPC
        rpc_url: String,

        #[arg(short, long)]
        /// URL for solana Websocket
        ws_url: String,
    },
    CopyPda {
        copy_program: String,
    },
}

#[tokio::main]
async fn main() -> io::Result<()> {
    let cli = Cli::parse();
    // localnet
    let _rpc_url = "http://localhost:8899";
    let socket_url = "ws://localhost:8900".to_string();
    let signer = read_keypair_file(signer).unwrap();
    let client = Client::new(socket_url, signer);

    // let mut threads = Vec::new();
    match &cli.command {
        Commands::CopyTransaction {
            copy_program,
            account_for_proof,
            signer,
            rpc_url,
            ws_url,
        } => {
            let account_for_proof = Pubkey::from_str(account_for_proof).unwrap();
            let account_state_from_rpc = client.query_account(&account_for_proof);

            let monitor_handle = std::thread::spawn(move || {
                let rt = Runtime::new().unwrap(); // Create a new Tokio runtime
                rt.block_on(
                    client.monitor_and_verify_updates(&account_for_proof, &account_state_from_rpc),
                )
                .unwrap(); // Run the async function `monitor_updates` to completion
            });

            // let copy_client = CopyClient::new(
            //     rpc_url.to_string(),
            //     ws_url.to_string(),
            //     signer_keypair,
            //     copy_program,
            // );
            client.send_transaction(copy_program, &account_for_proof).unwrap();
            monitor_handle.join().unwrap();
        }
        Commands::CopyPda { copy_program } => {
            let copy_program_pubkey = Pubkey::from_str(copy_program).unwrap();
            let (copy_pda, _) =
                Pubkey::find_program_address(&[PREFIX.as_bytes()], &copy_program_pubkey);
            println!("account: {}", copy_pda);
        }
    }

    Ok(())
}
