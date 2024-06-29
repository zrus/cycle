mod error;
mod keypair;

pub use error::Error;
pub use keypair::KeyPair;

use ring::rand;
use ring::rand::SecureRandom;
use ring::{aead, agreement, hkdf};

pub type Result<T, E = Error> = core::result::Result<T, E>;

// Encrypt a message using the recipient's public key
pub fn encrypt(
  recipient_public_key: impl AsRef<[u8]>,
  message: impl AsRef<[u8]>,
) -> Result<Vec<u8>> {
  let rng = rand::SystemRandom::new();

  // Generate an ephemeral key pair for this encryption
  let ephemeral_private_key =
    agreement::EphemeralPrivateKey::generate(&agreement::X25519, &rng)
      .map_err(|_| Error::EncryptFailed)?;
  let ephemeral_public_key = ephemeral_private_key
    .compute_public_key()
    .map_err(|_| Error::EncryptFailed)?;

  // Perform key agreement
  let shared_secret = agreement::agree_ephemeral(
    ephemeral_private_key,
    &agreement::UnparsedPublicKey::new(
      &agreement::X25519,
      recipient_public_key.as_ref(),
    ),
    |shared_key_material| shared_key_material.to_vec(),
  )
  .map_err(|_| Error::EncryptFailed)?;

  // Derive encryption key using HKDF
  let salt = hkdf::Salt::new(hkdf::HKDF_SHA256, &[]);
  let prk = salt.extract(&shared_secret);
  let mut aead_key = [0u8; 32];
  prk
    .expand(&[b"encryption"], &aead::CHACHA20_POLY1305)
    .map_err(|_| Error::EncryptFailed)?
    .fill(&mut aead_key)
    .map_err(|_| Error::EncryptFailed)?;

  // Encrypt the message
  let key = aead::UnboundKey::new(&aead::CHACHA20_POLY1305, &aead_key)
    .map_err(|_| Error::EncryptFailed)?;
  let sealing_key = aead::LessSafeKey::new(key);

  let mut nonce = [0u8; 12];
  rng.fill(&mut nonce).map_err(|_| Error::EncryptFailed)?;

  let mut in_out = message.as_ref().to_vec();
  sealing_key
    .seal_in_place_append_tag(
      aead::Nonce::assume_unique_for_key(nonce),
      aead::Aad::empty(),
      &mut in_out,
    )
    .map_err(|_| Error::EncryptFailed)?;

  // Combine ephemeral public key, nonce, and ciphertext
  let mut result = ephemeral_public_key.as_ref().to_vec();
  result.extend_from_slice(&nonce);
  result.extend(in_out);
  Ok(result)
}
