# Proof of Existence — Soroban Smart Contract

A lightweight Stellar Soroban smart contract that cryptographically proves the existence and integrity of data **without storing the original data** on a public network.

## Use Case

Designed for sensitive medical data (e.g., arrhythmia detection results from wearable devices like HaloBand). The flow:

1. **Off-chain**: Sensitive data is hashed using SHA-256 on the client side
2. **On-chain**: Only the hash value is submitted and stored on the Stellar ledger
3. **Verification**: Anyone can prove data has never been tampered with by comparing hashes

## Project Structure

```text
.
├── contracts
│   └── proof-of-existence
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       ├── Cargo.toml
│       └── Makefile
├── Cargo.toml
├── Cargo.lock
└── README.md
```

- The main contract lives in `contracts/proof-of-existence/src/lib.rs`
- Unit tests are in `contracts/proof-of-existence/src/test.rs`
- Each contract has its own `Cargo.toml` that relies on the top-level `Cargo.toml` workspace for dependencies

## Contract Functions

| Function | Parameters | Returns | Description |
|----------|------------|---------|-------------|
| `commit` | `hash: String`, `label: String` | `String` | Store a hash on the ledger. Rejects duplicates. |
| `verify` | `hash: String` | `String` | Check whether a hash has ever been committed. |
| `get_proof` | `hash: String` | `Option<Proof>` | Retrieve the full `Proof` struct by hash. |

### `Proof` Struct

```rust
pub struct Proof {
    pub hash: String,       // SHA-256 hash of the medical data
    pub label: String,      // Human-readable label (e.g., "ECG-2025-05-18")
    pub timestamp: u64,     // Ledger timestamp at time of commit (Unix)
}
```

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) v1.84.0+
- [Stellar CLI](https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)
- WASM target: `rustup target add wasm32v1-none`

Or use it directly in the browser at [soroban.studio](https://soroban.studio) — no local setup required.

### Build

```bash
stellar contract build
```

### Test

```bash
cargo test
```

### Generate Identity & Fund (Testnet)

```bash
stellar keys generate zugzwang --network testnet --fund
stellar keys address zugzwang
```

### Deploy to Testnet

```bash
stellar contract deploy \
  --wasm /app/target/wasm32v1-none/release/proof_of_existence.wasm \
  --source zugzwang \
  --network testnet \
  --alias proof-of-existence
```

## Usage Examples

### Commit a medical data hash

```bash
stellar contract invoke --id proof-of-existence --source zugzwang --network testnet -- commit --hash "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855" --label "HaloBand-ECG-Arrhythmia-Patient01"
```

### Verify a hash (original data unchanged)

```bash
stellar contract invoke --id proof-of-existence --source zugzwang --network testnet -- verify --hash "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
```

Expected output:
```
"VALID: Data ditemukan di ledger"
```

### Verify a fake hash (simulating tampered data)

```bash
stellar contract invoke --id proof-of-existence --source zugzwang --network testnet -- verify --hash "0000000000000000000000000000000000000000000000000000000000000000"
```

Expected output:
```
"INVALID: Hash tidak ditemukan"
```

### Retrieve full proof details

```bash
stellar contract invoke --id proof-of-existence --source zugzwang --network testnet -- get_proof --hash "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
```

## Test Coverage

`test.rs` covers 3 test scenarios:

| Test | Description |
|------|-------------|
| `test_commit_and_verify_success` | Commits a valid hash, verifies the response string, and validates the full `Proof` struct (hash, label, timestamp) |
| `test_prevent_duplicate_commit` | Contract rejects a second commit attempt using the same hash |
| `test_verify_non_existent_data` | Both `verify` and `get_proof` return negative results for a hash that was never committed |

## Security Notes

- The original medical data **never leaves the user's device**
- Only a SHA-256 hash (32 bytes) is stored on-chain
- Storage uses `persistent` — data remains available for the duration of the ledger TTL
- Duplicate hashes are rejected by the contract, preventing any overwrite of existing proofs

## Network

Developed and tested on **Stellar Testnet**.  
RPC Endpoint: `https://soroban-testnet.stellar.org`