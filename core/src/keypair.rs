use base64::engine::general_purpose;
use base64::Engine;
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::{RsaPrivateKey, RsaPublicKey};

use crate::{Error, Result};

const BIT_SIZE: usize = 32 * 8;

#[derive(Debug)]
pub struct KeyPair {
  private_key: RsaPrivateKey,
  public_key: RsaPublicKey,
}

impl KeyPair {
  pub fn generate() -> Result<Self> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, BIT_SIZE)
      .map_err(|_| Error::GenerateKeyFailed)?;
    let public_key = RsaPublicKey::from(&private_key);
    Ok(Self {
      private_key,
      public_key,
    })
  }

  pub fn public_key_base64(&self) -> Result<String> {
    let Ok(doc) = self.public_key.to_pkcs1_der() else {
      return Err(Error::UnableToParseToPem);
    };
    Ok(general_purpose::STANDARD.encode(doc.as_ref()))
  }

  pub fn private_key_base64(&self) -> Result<String> {
    let Ok(doc) = self.private_key.to_pkcs1_der() else {
      return Err(Error::UnableToParseToPem);
    };
    Ok(general_purpose::STANDARD.encode(doc.as_bytes()))
  }

  pub fn public_key_pem(&self) -> Result<String> {
    self
      .public_key
      .to_pkcs1_pem(LineEnding::LF)
      .map_err(|_| Error::UnableToParseToPem)
  }

  pub fn private_key_pem(&self) -> Result<String> {
    self
      .private_key
      .to_pkcs1_pem(LineEnding::LF)
      .map(|pem| pem.to_string())
      .map_err(|_| Error::UnableToParseToPem)
  }
}

#[test]
fn test_gen_keypair() {
  let key = KeyPair::generate();
  println!("{}", key.as_ref().unwrap().public_key_base64().unwrap());
  println!("{}", key.as_ref().unwrap().private_key_base64().unwrap());
}

#[test]
fn test_gen_keypair_to_pem() {
  let key = KeyPair::generate();
  println!("{}", key.as_ref().unwrap().public_key_pem().unwrap());
  println!("{}", key.as_ref().unwrap().private_key_pem().unwrap());
}
