use hmac::{Hmac, Mac};
use rsa::RsaPrivateKey;
use rsa::pkcs8::EncodePrivateKey;
// use rsa::pss::BlindedSigningKey;
// use rsa::signature::RandomizedSigner;
use sha2::Sha256;
/**********************************************************/
// 权限
/**********************************************************/
#[derive(Debug, Clone)]
pub struct AuthService;

impl AuthService {
    // pub fn rsa_signature(msg: &str, private_key: &RsaPrivateKey) -> String {
    //     let signing_key = BlindedSigningKey::<Sha256>::new(private_key.clone());
    //     let mut rng = rand::thread_rng();

    //     let signature = signing_key.sign_with_rng(&mut rng, msg.as_bytes());

    //     signature.to_string()
    // }

    pub fn hmac_sha(msg: &str, private_key: &RsaPrivateKey) -> String {
        type HmacSha256 = Hmac<Sha256>;
        let private_key = RsaPrivateKey::to_pkcs8_der(private_key).unwrap();

        let mut mac = HmacSha256::new_from_slice(private_key.as_bytes()).unwrap();
        mac.update(msg.as_bytes());

        let result = mac.finalize().into_bytes();

        hex::encode_upper(result)
    }
}

/**********************************************************/
// JWT
/**********************************************************/
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]

pub struct JwtPayload {
    pub admin_id: i32,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Jwttoken {
    pub payload: JwtPayload,
    pub exp: usize,
}

impl Jwttoken {
    pub fn generate_jwt(payload: JwtPayload, private_pem: &EncodingKey) -> anyhow::Result<String> {
        let exp = (Utc::now() + Duration::days(7)).timestamp() as usize;
        let claims = Jwttoken {
            payload: payload,
            exp,
        };
        //
        let token = jsonwebtoken::encode(&Header::new(Algorithm::RS256), &claims, &private_pem)
            .map_err(|e| anyhow::anyhow!("jwt encode failed: {}", e))?;
        Ok(token)
    }

    pub fn verify_jwt(token: &str, public_pem: &DecodingKey) -> Option<Jwttoken> {
        let data =
            decode::<Jwttoken>(token, &public_pem, &Validation::new(Algorithm::RS256)).ok()?;
        Some(data.claims)
    }
}
