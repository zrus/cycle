use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::{Error, Result};

/// Decryptor
///
/// Decrypt ciphertext by using [RsaPrivateKey]
pub struct Decryptor {
  pem: RsaPrivateKey,
}

impl Decryptor {
  /// Initialize Decryptor with private key in pem format
  pub fn new(private_key: impl AsRef<str>) -> Result<Self> {
    Ok(Self {
      pem: RsaPrivateKey::from_pkcs1_pem(private_key.as_ref())
        .map_err(|_| Error::UnableToReadPem)?,
    })
  }

  /// Initialize Decryptor with private key
  pub fn from_der(private_key: impl AsRef<[u8]>) -> Result<Self> {
    Ok(Self {
      pem: RsaPrivateKey::from_pkcs1_der(private_key.as_ref())
        .map_err(|_| Error::UnableToReadDer)?,
    })
  }

  /// Decrypt ciphertext
  pub fn decrypt(&self, ciphertext: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    self
      .pem
      .decrypt(Pkcs1v15Encrypt, ciphertext.as_ref())
      .map_err(|_| Error::DecryptFailed)
  }
}
