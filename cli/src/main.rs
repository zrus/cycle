use clap::{Args, Parser, Subcommand};

#[derive(Subcommand)]
enum Command {
  /// Generate key pair
  KeyGen(KeyGenOption),
  /// Encrypt message
  #[clap(short_flag = 'e')]
  Encrypt(EncryptArgs),
  /// Decrypt message
  #[clap(short_flag = 'd')]
  Decrypt(DecryptArgs),
}

#[derive(Args)]
struct KeyGenOption {
  /// Print .pem format
  #[clap(short = 'p', long = "pem")]
  to_pem: bool,
}

#[derive(Args)]
struct EncryptArgs {
  /// Public key
  #[clap(short = 'p', long)]
  public_key: String,
  /// Message to encrypt
  #[clap(short = 'm', long)]
  message: String,
}

#[derive(Args)]
struct DecryptArgs {
  /// Private key
  #[clap(short = 'p', long)]
  private_key: String,
  /// Ciphertext to decrypt
  #[clap(short = 'c', long)]
  cipher: String,
}

#[derive(Parser)]
struct Cli {
  #[command(subcommand)]
  command: Command,
}

fn main() -> core::Result<()> {
  let cli = Cli::parse();

  match cli.command {
    Command::KeyGen(KeyGenOption{ to_pem }) => {
      let keypair = core::KeyPair::generate()?;
      if to_pem {
        println!("Public key:\n{}", keypair.public_key_pem()?);
        println!();
        println!("Private key:\n{}", keypair.private_key_pem()?);
      } else {
        println!("Public key: {}", keypair.public_key_base64()?);
        println!();
        println!("Private key: {}", keypair.private_key_base64()?);
      }
    }
    Command::Encrypt(EncryptArgs{ public_key, message }) => {
      println!("{}", core::encrypt(public_key, message)?);
    }
    Command::Decrypt(DecryptArgs{ private_key, cipher }) => {
      println!("{}", core::decrypt(private_key, cipher)?);
    }
  }
  Ok(())
}