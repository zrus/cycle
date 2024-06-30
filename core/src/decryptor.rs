use rsa::pkcs1::DecodeRsaPrivateKey;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};

use crate::{Error, Result};

pub struct Decryptor {
  pem: RsaPrivateKey,
}

impl Decryptor {
  pub fn new(private_key: impl AsRef<str>) -> Result<Self> {
    Ok(Self {
      pem: RsaPrivateKey::from_pkcs1_pem(private_key.as_ref())
        .map_err(|_| Error::UnableToReadPem)?,
    })
  }

  pub fn from_der(private_key: impl AsRef<[u8]>) -> Result<Self> {
    Ok(Self {
      pem: RsaPrivateKey::from_pkcs1_der(private_key.as_ref())
        .map_err(|_| Error::UnableToReadDer)?,
    })
  }

  pub fn decrypt(&self, ciphertext: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    self
      .pem
      .decrypt(Pkcs1v15Encrypt, ciphertext.as_ref())
      .map_err(|_| Error::DecryptFailed)
  }
}
