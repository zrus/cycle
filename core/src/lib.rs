mod decryptor;
mod encryptor;
mod error;
mod keypair;

use base64::engine::general_purpose;
use base64::Engine;
pub use decryptor::Decryptor;
pub use encryptor::Encryptor;
pub use error::Error;
pub use keypair::KeyPair;

pub type Result<T, E = Error> = core::result::Result<T, E>;

/// Encrypt message
pub fn encrypt(
  public_key: impl AsRef<str>,
  message: impl AsRef<[u8]>,
) -> Result<String> {
  let public_key = general_purpose::STANDARD
    .decode(public_key.as_ref())
    .map_err(|_| Error::DecodeKeyFailed)?;
  let encryptor = Encryptor::from_der(public_key)?;
  let encrypted = encryptor.encrypt(message)?;
  Ok(general_purpose::STANDARD.encode(encrypted))
}

/// Decrypt ciphertext
pub fn decrypt(
  private_key: impl AsRef<str>,
  cipher: impl AsRef<[u8]>,
) -> Result<String> {
  let cipher = general_purpose::STANDARD
    .decode(cipher)
    .map_err(|_| Error::UnableToParseCipher)?;
  let private_key = general_purpose::STANDARD
    .decode(private_key.as_ref())
    .map_err(|_| Error::DecodeKeyFailed)?;
  let decryptor = Decryptor::from_der(private_key)?;
  let decrypted = decryptor.decrypt(cipher)?;
  String::from_utf8(decrypted).map_err(|_| Error::UnableToParseCipher)
}

#[test]
fn test_encrypt_decrypt() -> Result<()> {
  let keypair = KeyPair::generate()?;
  let text = "Hello, World!";
  let encrypted = encrypt(keypair.public_key_base64()?, text)?;
  println!("{encrypted}");
  let decrypted = decrypt(keypair.private_key_base64()?, encrypted)?;
  assert_eq!(decrypted, text);
  Ok(())
}
