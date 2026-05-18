#![cfg(test)]

use super::*;
use soroban_sdk::{Env, String};

#[test]
fn test_commit_and_verify_success() {
    // 1. Setup environment Soroban lokal
    let env = Env::default();
    
    // 2. Registrasi kontrak ke ledger virtual
    let contract_id = env.register_contract(None, ProofOfExistence);
    let client = ProofOfExistenceClient::new(&env, &contract_id);

    // 3. Setup data uji
    let hash = String::from_str(&env, "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
    let label = String::from_str(&env, "HaloBand-ECG-Aritmia-Pasien01");

    // --- EKSEKUSI PENGUJIAN ---

    // A. Uji Commit
    let commit_result = client.commit(&hash, &label);
    assert_eq!(
        commit_result, 
        String::from_str(&env, "OK: Proof berhasil disimpan di ledger"),
        "Fungsi commit gagal mengembalikan pesan sukses yang benar"
    );

    // B. Uji Verifikasi Teks
    let verify_result = client.verify(&hash);
    assert_eq!(
        verify_result, 
        String::from_str(&env, "VALID: Data ditemukan di ledger"),
        "Fungsi verify gagal mendeteksi data yang sudah dimasukkan"
    );

    // C. Uji Detail Proof
    let proof = client.get_proof(&hash).unwrap();
    assert_eq!(proof.hash, hash, "Hash di dalam struct tidak cocok");
    assert_eq!(proof.label, label, "Label di dalam struct terdistorsi");
    assert!(proof.timestamp > 0, "Timestamp tidak terekam oleh ledger");
}

#[test]
fn test_prevent_duplicate_commit() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ProofOfExistence);
    let client = ProofOfExistenceClient::new(&env, &contract_id);

    let hash = String::from_str(&env, "hash_duplikat_123");
    let label_awal = String::from_str(&env, "Rekam_Medis_Asli");
    let label_palsu = String::from_str(&env, "Rekam_Medis_Palsu");

    // Eksekusi pertama (Harus sukses)
    client.commit(&hash, &label_awal);

    // Eksekusi kedua dengan hash yang sama (Harus gagal/ditolak)
    let duplicate_result = client.commit(&hash, &label_palsu);
    assert_eq!(
        duplicate_result, 
        String::from_str(&env, "ERROR: Hash sudah pernah dicommit"),
        "Kontrak gagal memblokir duplikasi hash!"
    );
}

#[test]
fn test_verify_non_existent_data() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ProofOfExistence);
    let client = ProofOfExistenceClient::new(&env, &contract_id);

    let fake_hash = String::from_str(&env, "hash_bodong_tidak_ada");

    // Uji fungsi verify
    let verify_result = client.verify(&fake_hash);
    assert_eq!(
        verify_result, 
        String::from_str(&env, "INVALID: Hash tidak ditemukan")
    );

    // Uji fungsi get_proof
    let proof_result = client.get_proof(&fake_hash);
    assert!(proof_result.is_none(), "Kontrak mengembalikan data untuk hash yang tidak pernah ada");
}