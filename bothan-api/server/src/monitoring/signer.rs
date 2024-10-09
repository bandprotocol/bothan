use ed25519::signature::Signer as _;
use ed25519_dalek::{SecretKey, SigningKey};
use hex::{FromHex, FromHexError, ToHex};
use rand::rngs::OsRng;

pub struct Signer {
    pub signing_key: SigningKey,
}

impl Signer {
    pub fn random() -> Self {
        Self {
            signing_key: SigningKey::generate(&mut OsRng),
        }
    }

    pub fn to_hex(&self) -> String {
        self.signing_key.to_bytes().encode_hex()
    }

    pub fn from_hex(hex: &str) -> Result<Self, FromHexError> {
        Ok(Self {
            signing_key: SigningKey::from_bytes(&SecretKey::from_hex(hex)?),
        })
    }

    // Get hex encoded public key
    pub fn public_key(&self) -> String {
        self.signing_key.verifying_key().to_bytes().encode_hex()
    }

    // Sign data
    pub fn sign(&self, data: &[u8]) -> Vec<u8> {
        self.signing_key.sign(data).to_vec()
    }
}
