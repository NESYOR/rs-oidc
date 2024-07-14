use rand::rngs::OsRng;
use pem::Pem;
use rsa::{pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey}, pkcs8::LineEnding, RsaPrivateKey};
use sshkeys::PublicKey;

pub fn generate_keypair() -> Result<(String,String),Box<dyn std::error::Error>> {
    let mut csprng = OsRng;
    let bits = 4096;
    let private_key = RsaPrivateKey::new(&mut csprng, bits)?;
    let private_key_pem = Pem::new("RSA PRIVATE KEY".to_string(),private_key.to_pkcs1_der().unwrap().as_bytes());
    let private_key_pem = pem::encode(&private_key_pem);
    let public_key = private_key.to_public_key();
    let public_key_ssh_format = PublicKey::from_string(&public_key.to_pkcs1_pem(LineEnding::LF)?.as_str())?;
    Ok((private_key_pem,public_key_ssh_format.to_string()))
    }