use hex::{self};
use sodiumoxide::crypto::sign::{gen_keypair, SecretKey};

#[derive(Debug,Clone)]
pub struct Wallet {
    pub priv_key: SecretKey,
    pub public_key: String,
}

impl Wallet {
    pub fn new() -> Wallet {
        let (pub_key,priv_key) = gen_keypair();

        let public_key = format!("0x{}",hex::encode(pub_key));

        Wallet{
            priv_key,
            public_key,
        }
    }
}
