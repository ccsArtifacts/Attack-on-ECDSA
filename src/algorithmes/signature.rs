use rug::Integer;
use rug::integer::Order;
use secp256k1::{All, Message, Secp256k1, SecretKey};
use sha2::{Digest, Sha256};

/// Signs a message and simulates MSB leakage of the nonce.
///
/// ⚠️ Educational only – nonce leakage is simulated.
pub fn sign_message(
    secp: &Secp256k1<All>,
    sk: &SecretKey,
    message: &[u8],
    msb: usize,
) -> (u64, Integer, Integer, Integer) {
    let hash = Sha256::digest(message);
    let digest: [u8; 32] = hash.into();

    let msg = Message::from_digest(digest);
    let sig = secp.sign_ecdsa(&msg, sk);

    let compact = sig.serialize_compact();
    let r = Integer::from_digits(&compact[0..32], Order::MsfBe);
    let s = Integer::from_digits(&compact[32..64], Order::MsfBe);
    let h = Integer::from_digits(&digest, Order::MsfBe);

    let mut k_sim = u64::from_be_bytes(digest[0..8].try_into().unwrap());
    if k_sim == 0 {
        k_sim = 1;
    }

    let take = msb.min(64);
    let k_msb = if take == 0 { 0 } else { k_sim >> (64 - take) };

    (k_msb, h, r, s)
}
