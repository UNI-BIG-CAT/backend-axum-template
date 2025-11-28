use jsonwebtoken::{DecodingKey, EncodingKey};
use rsa::{RsaPrivateKey, RsaPublicKey, pkcs8::DecodePrivateKey, pkcs8::DecodePublicKey};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct RsaKey {
    pub pw_private: RsaPrivateKey,
    pub _pw_public: RsaPublicKey,
    pub jwt_private: EncodingKey,
    pub jwt_public: DecodingKey,
}

pub fn get_rsa_key() -> RsaKey {
    // 读取rsa-key
    let pem = fs::read_to_string(Path::new("config/key").join("password-private-key.pem"))
        .expect("pw_private_key pem read failed!");
    let pw_private_key = RsaPrivateKey::from_pkcs8_pem(&pem).expect("pw_private_key parse failed!");
    //
    let pem = fs::read_to_string(Path::new("config/key").join("password-public-key.pem"))
        .expect("pw_public_key pem read failed!");
    let pw_public_key =
        RsaPublicKey::from_public_key_pem(&pem).expect("pw_public_key parse failed!");
    //
    let jwt_private_key = fs::read_to_string(Path::new("config/key").join("jwt-private-key.pem"))
        .expect("jwt_private_key pem read failed!");
    let jwt_private_key = EncodingKey::from_rsa_pem(jwt_private_key.as_bytes())
        .expect("jwt_private_key parse failed!");
    //
    let jwt_public_key = fs::read_to_string(Path::new("config/key").join("jwt-public-key.pem"))
        .expect("jwt_public_key pem read failed!");
    let jwt_public_key =
        DecodingKey::from_rsa_pem(jwt_public_key.as_bytes()).expect("jwt_public_key parse failed!");
    RsaKey {
        pw_private: pw_private_key,
        _pw_public: pw_public_key,
        jwt_private: jwt_private_key,
        jwt_public: jwt_public_key,
    }
}
