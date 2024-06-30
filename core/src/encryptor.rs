use rsa::pkcs1::DecodeRsaPublicKey;
use rsa::{Pkcs1v15Encrypt, RsaPublicKey};

use crate::{Error, Result};

pub struct Encryptor {
  pem: RsaPublicKey,
}

impl Encryptor {
  pub fn new(public_key: impl AsRef<str>) -> Result<Self> {
    Ok(Self {
      pem: RsaPublicKey::from_pkcs1_pem(public_key.as_ref())
        .map_err(|_| Error::UnableToReadPem)?,
    })
  }

  pub fn from_der(public_key: impl AsRef<[u8]>) -> Result<Self> {
    Ok(Self {
      pem: RsaPublicKey::from_pkcs1_der(public_key.as_ref())
        .map_err(|_| Error::UnableToReadDer)?,
    })
  }

  pub fn encrypt(&self, message: impl AsRef<[u8]>) -> Result<Vec<u8>> {
    let mut rng = rand::thread_rng();
    self
      .pem
      .encrypt(&mut rng, Pkcs1v15Encrypt, message.as_ref())
      .map_err(|_| Error::EncryptFailed)
  }
}
