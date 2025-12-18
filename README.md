# attack-on-ecdsa

Educational Rust crate demonstrating classical attacks against ECDSA
when cryptographic assumptions are violated (e.g. nonce reuse).

## ⚠️ Disclaimer
This crate is for educational and research purposes only.
Do NOT use in production systems.

## Features
- ECDSA key generation
- Signature simulation
- Private key recovery attacks

## Example

'''rust 
use attack_on_ecdsa::break_ecdsa::recover_private_key;
