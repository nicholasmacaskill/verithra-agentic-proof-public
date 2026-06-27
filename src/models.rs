use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicInputs {
    pub claimed_sharpe_ratio: f64,
    pub min_sharpe_threshold: f64,
    pub num_trades: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    pub trades_commitment: String, // Hex-encoded hash of the secret trade records
    pub public_inputs: PublicInputs,
    pub proof_signature: String,   // Simulated ZK-SNARK proof signature
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claim {
    pub claimed_sharpe_ratio: f64,
    pub min_sharpe_threshold: f64,
    pub num_trades: usize,
    pub description: String,
}
