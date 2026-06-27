use crate::models::{Proof, Claim, PublicInputs};
use sha2::{Sha256, Digest};

pub const PROOF_SALT: &str = "VERITHRA_PROOF_OF_ALPHA_ZK_SALT_v1";

/// Computes the expected proof signature to verify cryptographic integrity.
pub fn compute_proof_signature(commitment: &str, public_inputs: &PublicInputs) -> String {
    let mut hasher = Sha256::new();
    hasher.update(commitment.as_bytes());
    hasher.update(format!("{:.6}", public_inputs.claimed_sharpe_ratio).as_bytes());
    hasher.update(format!("{:.6}", public_inputs.min_sharpe_threshold).as_bytes());
    hasher.update(public_inputs.num_trades.to_be_bytes());
    hasher.update(PROOF_SALT.as_bytes());
    hex::encode(hasher.finalize())
}

/// Verifies a mock ZK-SNARK performance proof and its associated claim.
/// Returns true if the proof is cryptographically and mathematically valid.
pub fn verify_proof_and_claim(proof: &Proof, claim: &Claim) -> bool {
    // 1. Verify that the public inputs match the claim
    let matches_claim = (proof.public_inputs.claimed_sharpe_ratio - claim.claimed_sharpe_ratio).abs() < 1e-6
        && (proof.public_inputs.min_sharpe_threshold - claim.min_sharpe_threshold).abs() < 1e-6
        && proof.public_inputs.num_trades == claim.num_trades;

    if !matches_claim {
        println!("[Verifier Error] Public inputs in the proof do not match the public claim.");
        return false;
    }

    // 2. Verify that the claimed Sharpe Ratio satisfies the threshold
    if proof.public_inputs.claimed_sharpe_ratio < proof.public_inputs.min_sharpe_threshold - 1e-6 {
        println!(
            "[Verifier Error] Claimed Sharpe Ratio ({:.6}) is less than the required threshold ({:.6}).",
            proof.public_inputs.claimed_sharpe_ratio,
            proof.public_inputs.min_sharpe_threshold
        );
        return false;
    }

    // 3. Verify that the number of trades satisfies the minimum requirement (100 trades)
    if proof.public_inputs.num_trades < 100 {
        println!(
            "[Verifier Error] Number of trades ({}) is less than the required minimum of 100.",
            proof.public_inputs.num_trades
        );
        return false;
    }

    // 4. Verify the cryptographic ZK proof signature (Fiat-Shamir simulation)
    let expected_signature = compute_proof_signature(
        &proof.trades_commitment,
        &proof.public_inputs
    );

    if expected_signature != proof.proof_signature {
        println!("[Verifier Error] Cryptographic proof signature is invalid! The proof may have been tampered with.");
        return false;
    }

    println!("[Verifier Success] Cryptographic proof and performance claim are VALID.");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invalid_tampered_signature() {
        let public_inputs = PublicInputs {
            claimed_sharpe_ratio: 2.5,
            min_sharpe_threshold: 2.0,
            num_trades: 120,
        };
        let commitment = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef".to_string();
        
        let proof = Proof {
            trades_commitment: commitment,
            public_inputs,
            proof_signature: "wrong_sig_value".to_string(),
        };

        let claim = Claim {
            claimed_sharpe_ratio: 2.5,
            min_sharpe_threshold: 2.0,
            num_trades: 120,
            description: "Test".to_string(),
        };

        assert!(!verify_proof_and_claim(&proof, &claim));
    }
}
