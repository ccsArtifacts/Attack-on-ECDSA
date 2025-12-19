use rand::RngCore;
use secp256k1::{PublicKey, Secp256k1, SecretKey};

/// Generates an ECDSA key pair (secp256k1).
pub fn key_gen() -> (SecretKey, PublicKey) {
    let secp = Secp256k1::new();
    let mut rng = rand::rng();

    let sk = loop {
        let mut bytes = [0u8; 32];
        rng.fill_bytes(&mut bytes);
        if let Ok(sk) = SecretKey::from_slice(&bytes) {
            break sk;
        }
    };

    let pk = PublicKey::from_secret_key(&secp, &sk);
    (sk, pk)
}
