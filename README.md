# Verithra Public Verifier (Proof of Alpha)

**Verithra is the cryptographic trust layer for independent quants, autonomous trading daemons, and prediction market syndicates.**

This repository contains the **Public Verifier** engine. It allows investors, syndicate members, and third parties to trustlessly verify cryptographic performance claims (e.g. Sharpe Ratio, PnL, and trade count) without requiring access to the trader's private execution logs, API credentials, or wallet addresses.

---

## How It Works

1. **Local Proof Generation (Private):**
   Traders run a private client that ingests their raw trade history (from decentralized protocols like Polymarket or off-chain APIs via zkTLS). The client computes performance metrics and generates a cryptographic zero-knowledge-style proof (`proof.json`) alongside a signed public performance statement (`claim.json`).

2. **Public Verification (This Repo):**
   Investors use this public verifier tool to validate that the cryptographic proof mathematically corresponds to the claimed performance metrics, ensuring authenticity without exposing the "secret sauce" of the trading strategy.

```
                  ┌──────────────────────────────┐
                  │ Private Trade Records (CEX)  │
                  └──────────────┬───────────────┘
                                 │
                                 ▼ (Prover circuit)
┌────────────┐    ┌──────────────────────────────┐
│ claim.json │ ◄──┤    Cryptographic Prover      │
└─────┬──────┘    └──────────────┬───────────────┘
      │                          │
      │   ┌────────────┐         ▼
      └──►│ proof.json ├─────────┐
          └─────┬──────┘         │
                │                ▼
                │    ┌───────────────────────┐
                └───►│   Public Verifier     │
                     └───────────┬───────────┘
                                 │
                                 ▼
                     [ SUCCESS / FAILURE ]
```

---

## Getting Started

### Prerequisites

You need [Rust and Cargo](https://rustup.rs/) installed to build and run the verifier.

### Installation

Clone this repository and compile the verifier:

```bash
git clone https://github.com/nicholasmacaskill/verithra-agentic-proof-public.git
cd verithra-agentic-proof-public
cargo build --release
```

### Running Verification

To verify a performance claim, run the executable with paths to the public proof and claim files:

```bash
cargo run --release -- verify --proof <path_to_proof.json> --claim <path_to_claim.json>
```

#### Example

```bash
cargo run --release -- verify --proof proof.json --claim claim.json
```

---

## Technical Details

- **Cryptographic Commitment:** The verifier validates that the secret trade dataset matches a hash commitment published in `proof.json`.
- **Stat Verification:** The Sharpe Ratio, minimum trade constraints, and other metrics are recalculated using deterministic logic matching the zero-knowledge circuit rules.
- **Tamper Protection:** A Fiat-Shamir-based cryptographic signature guarantees that neither the public inputs nor the commitments have been altered.

---

## License

This project is licensed under the MIT License. See [LICENSE](LICENSE) for details.
