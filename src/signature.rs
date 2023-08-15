use hex::{FromHex, ToHex};
use hmac::{Hmac, Mac};
use md5::{Digest, Md5};
use sha2::Sha256;
use std::collections::HashMap;

type HmacSha256 = Hmac<Sha256>;

pub fn create_body_md5(body: &str) -> String {
    let mut sh = Md5::new();
    sh.update(body.as_bytes());
    let result = sh.finalize();
    result.encode_hex()
}

pub fn create_channel_auth<'a>(
    auth_map: &mut HashMap<&'a str, String>,
    key: &str,
    secret: &str,
    to_sign: &str,
) -> Option<()> {
    let auth_signature = create_auth_signature(to_sign, secret)?;
    let auth_string = format!("{}:{}", key, auth_signature);
    auth_map.insert("auth", auth_string);
    Some(())
}

pub fn check_signature(signature: &str, secret: &str, body: &str) -> bool {
    let mut hmac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("HMAC can take key of any size (error {e})");
            return false;
        }
    };
    hmac.update(body.as_bytes());
    let decoded_signature = match Vec::from_hex(signature) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error decoding signature: {e}");
            return false;
        }
    };
    hmac.verify_slice(&decoded_signature).is_ok()
}

pub fn create_auth_signature<'a>(to_sign: &str, secret: &'a str) -> Option<String> {
    let mut hmac = match HmacSha256::new_from_slice(secret.as_bytes()) {
        Ok(h) => h,
        Err(e) => {
            eprintln!("HMAC can take key of any size (error {e})");
            return None;
        }
    };
    hmac.update(to_sign.as_bytes());
    let result = hmac.finalize();
    let code = result.into_bytes();
    Some(code.encode_hex())
}
