use std::fs::File;
use std::io::Read;
use clap::{Parser, Subcommand};
use verithra_verifier::models::{Proof, Claim};
use verithra_verifier::verifier::verify_proof_and_claim;

#[derive(Parser)]
#[command(name = "verithra-verifier")]
#[command(about = "Verithra Public Performance Verifier CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Verify a public performance claim against a generated proof file
    Verify {
        /// Path to the public proof.json
        #[arg(long, default_value = "proof.json")]
        proof: String,

        /// Path to the public claim.json
        #[arg(long, default_value = "claim.json")]
        claim: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Verify { proof, claim } => {
            println!("Verifying proof '{}' against claim '{}'...", proof, claim);

            // Read proof.json
            let mut proof_file = match File::open(&proof) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("[Error] Could not open proof file '{}': {}", proof, e);
                    std::process::exit(1);
                }
            };
            let mut proof_contents = String::new();
            proof_file.read_to_string(&mut proof_contents).expect("Failed to read proof file");
            let proof_data: Proof = match serde_json::from_str(&proof_contents) {
                Ok(p) => p,
                Err(e) => {
                    eprintln!("[Error] Failed to parse proof JSON: {}", e);
                    std::process::exit(1);
                }
            };

            // Read claim.json
            let mut claim_file = match File::open(&claim) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("[Error] Could not open claim file '{}': {}", claim, e);
                    std::process::exit(1);
                }
            };
            let mut claim_contents = String::new();
            claim_file.read_to_string(&mut claim_contents).expect("Failed to read claim file");
            let claim_data: Claim = match serde_json::from_str(&claim_contents) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("[Error] Failed to parse claim JSON: {}", e);
                    std::process::exit(1);
                }
            };

            // Verify
            let is_valid = verify_proof_and_claim(&proof_data, &claim_data);
            if is_valid {
                println!("\nVerification RESULT: SUCCESS (Proof is mathematically and cryptographically valid).");
            } else {
                println!("\nVerification RESULT: FAILED.");
                std::process::exit(1);
            }
        }
    }
}
