use ring::{agreement, rand};

use crate::{Error, Result};

// Key pair structure
pub struct KeyPair {
  pub(crate) private_key: agreement::EphemeralPrivateKey,
  pub(crate) public_key: Vec<u8>,
}

impl KeyPair {
  // Generate a new key pair
  pub fn new() -> Result<Self, Error> {
    let rng = rand::SystemRandom::new();
    let private_key =
      agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
        .map_err(|_| Error::GenerateKeyFailed)?;
    let public_key = private_key
      .compute_public_key()
      .map_err(|_| Error::GenerateKeyFailed)?
      .as_ref()
      .to_vec();
    Ok(Self {
      private_key,
      public_key,
    })
  }

  pub fn public_key(&self) -> &[u8] {
    &self.public_key
  }
}
