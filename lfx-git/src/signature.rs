// Copyright © 2023 Linear Fox. This file is under GPLv3 License.

use ring::signature::{Ed25519KeyPair, Signature};
use std::error::Error;

pub struct SignatureUtil;

impl SignatureUtil {
    pub fn sign_message(message: &[u8], key_pair: &Ed25519KeyPair) -> Result<Signature, Box<dyn Error>> {
        let signature = key_pair.sign(message);
        Ok(signature)
    }

    pub fn verify_signature(message: &[u8], signature: &Signature, public_key: &[u8]) -> Result<(), Box<dyn Error>> {
        let public_key = ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        public_key.verify(message, signature.as_ref())?;
        Ok(())
    }

    pub fn generate_key_pair() -> Result<Ed25519KeyPair, Box<dyn Error>> {
        let rng = ring::rand::SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rng)?;
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref())?;
        Ok(key_pair)
    }
}