#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol};

// Key untuk storage
const PROOFS: Symbol = symbol_short!("PROOFS");

// Struct satu proof
#[contracttype]
pub struct Proof {
    pub hash: String,       // SHA-256 hash dari data medis
    pub label: String,      // label/keterangan (misal: "ECG-2025-05-18")
    pub timestamp: u64,     // waktu ledger saat disimpan
}

#[contract]
pub struct ProofOfExistence;

#[contractimpl]
impl ProofOfExistence {

    // Simpan hash ke ledger (data asli TIDAK pernah masuk sini)
    pub fn commit(env: Env, hash: String, label: String) -> String {
        // Cek apakah hash ini sudah pernah di-commit sebelumnya
        let key = hash.clone();
        if env.storage().persistent().has(&key) {
            return String::from_str(&env, "ERROR: Hash sudah pernah dicommit");
        }

        let proof = Proof {
            hash: hash.clone(),
            label,
            timestamp: env.ledger().timestamp(),
        };

        // Simpan dengan key = hash itu sendiri
        env.storage().persistent().set(&key, &proof);

        String::from_str(&env, "OK: Proof berhasil disimpan di ledger")
    }

    // Verifikasi: apakah hash ini pernah dicommit? Kapan?
    pub fn verify(env: Env, hash: String) -> String {
        let key = hash.clone();
        match env.storage().persistent().get::<String, Proof>(&key) {
            Some(_proof) => {
                String::from_str(&env, "VALID: Data ditemukan di ledger")
            }
            None => String::from_str(&env, "INVALID: Hash tidak ditemukan"),
        }
    }

    // Ambil detail proof berdasarkan hash
    pub fn get_proof(env: Env, hash: String) -> Option<Proof> {
        env.storage().persistent().get::<String, Proof>(&hash)
    }
}
mod test;