extern crate hmac;
extern crate jwt;
extern crate sha2;

use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha256;
use std::collections::BTreeMap;

pub struct JwtToken {
    pub user_id: i32,
    pub body: String,
}

impl JwtToken {
    pub fn encode(user_id: i32) -> String {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();

        let mut claims = BTreeMap::new();
        claims.insert("user_id", user_id);
        
        claims.sign_with_key(&key).unwrap()
    }

    pub fn decode(encoded_token: String) -> Result<JwtToken, &'static str> {
        //-> Result<JwtToken, &'static str> {
        let key: Hmac<Sha256> = Hmac::new_from_slice(b"secret").unwrap();
        let token: Result<BTreeMap<String, i32>, jwt::Error> = encoded_token.verify_with_key(&key);
        match token {
            Ok(claims) => Ok(JwtToken {user_id: claims["user_id"], body: encoded_token}),
            Err(_) => Err("Token decode error")
        }
    }
}

#[cfg(test)]
mod jwt_test {
    use super::JwtToken;
    #[test]
    fn jwt_encode() {
        let token = JwtToken::encode(1);
        assert_eq!(
            token,
            "eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxfQ.ETUYUOkmfnWsWIvA8iBOkE2s1ZQ0V_zgnG_c4QRrhbg"
        );
    }

    #[test]
    fn jwt_decode() {
        let token = "eyJhbGciOiJIUzI1NiJ9.eyJ1c2VyX2lkIjoxfQ.ETUYUOkmfnWsWIvA8iBOkE2s1ZQ0V_zgnG_c4QRrhbg";
        let data = JwtToken::decode(String::from(token)).unwrap();
        assert_eq!(data.user_id, 1);
        assert_eq!(data.body, token);
    }

    #[test]
    #[should_panic(expected="Token decode error")]
    fn jwt_decode_incorrect_token() {
        let token = "incorrect_token";
        let data = JwtToken::decode(String::from(token)).unwrap();
        assert_eq!(data.user_id, 1);
        assert_eq!(data.body, token);
    }
}
