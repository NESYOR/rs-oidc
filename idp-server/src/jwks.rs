use idp_server::Server;
use serde::{Deserialize, Serialize};
use idp_server::traits::JwksHandler;

#[derive(Serialize, Deserialize)]
struct JwkKey {
    kty: String,
    use_: String,
    kid: String,
    n: String,
    e: String,
    alg: String,
}

#[derive(Serialize, Deserialize)]
struct Jwks{
    keys: Vec<JwkKey>
}

impl JwksHandler for Server{

    async fn jwks(&self)  -> impl Responder {
        let private_key_pem = self.get_private_key().to_pem().unwrap();
        let rsa = Rsa::private_key_from_pem(&private_key_pem).expect("Failed to parse private key");
        let pkey = PKey::from_rsa(rsa).expect("Failed to create PKey");
    
        let rsa = pkey.rsa().expect("Failed to get RSA");
        let n = base64::encode(rsa.n().to_vec());
        let e = "AQAB"; // RSA public exponent e is usually 65537, which is AQAB in base64
    
        let jwks = Jwks {
            keys: vec![JwksKey {
                kid: "0-0-0-1".to_string(),
                alg: "RS256".to_string(),
                kty: "RSA".to_string(),
                use_: "sig".to_string(),
                n,
                e: e.to_string(),
            }],
        };
    
        let out = serde_json::to_string(&jwks).expect("Failed to serialize JWKS");
        HttpResponse::Ok().body(out)
    }
    
}





