use base64::{engine::general_purpose, Engine as _};

pub fn to_base64(s: &str) -> String {
    general_purpose::STANDARD.encode(s.as_bytes())
}

pub fn from_base64(s: &str) -> Vec<u8> {
    general_purpose::STANDARD.decode(s.as_bytes()).unwrap()
}
