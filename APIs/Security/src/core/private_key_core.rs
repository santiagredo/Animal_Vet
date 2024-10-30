use actix_web::http;
use once_cell::sync::OnceCell;
use openssl::{
    encrypt::{Decrypter, Encrypter},
    pkey::{PKey, Private},
    rsa::{Padding, Rsa},
};
use utils::{get_config, CodeMessage, Outcome};

use crate::data::PrivateKeyData;

static PRIVATE_KEY: OnceCell<PKey<Private>> = OnceCell::new();

pub struct PrivateKeyCore;

impl PrivateKeyCore {
    async fn select_private_key() -> Outcome<&'static PKey<Private>, CodeMessage, CodeMessage> {
        if let Some(key) = PRIVATE_KEY.get() {
            return Outcome::Success(key);
        }

        let priv_key_model =
            match PrivateKeyData::select_private_key(&get_config().await.settings_db_url).await {
                Outcome::Error(err) => return Outcome::Error(err),
                Outcome::Failure(fail) => return Outcome::Failure(fail),
                Outcome::Success(val) => val,
            };

        let private_key = match priv_key_model.key {
            None => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Private key is empty"),
                })
            }
            Some(val) => val,
        };

        let private_key = match Rsa::private_key_from_pem(private_key.as_bytes()) {
            Err(_) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to convert private key to bytes"),
                })
            }
            Ok(val) => val,
        };

        let private_pkey = match PKey::from_rsa(private_key) {
            Err(_) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: format!("Failed to convert private key bytes to pkey"),
                })
            }
            Ok(val) => val,
        };

        if PRIVATE_KEY.get().is_none() && PRIVATE_KEY.set(private_pkey).is_err() {
            return Outcome::Error(CodeMessage {
                http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to cache private key"),
            });
        }

        match PRIVATE_KEY.get() {
            None => Outcome::Error(CodeMessage {
                http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                message: format!("Failed to retreive cached private key"),
            }),
            Some(val) => Outcome::Success(val),
        }
    }

    pub async fn encrypt_content(content: String) -> Outcome<String, CodeMessage, CodeMessage> {
        let private_key = match Self::select_private_key().await {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let mut encrypter = match Encrypter::new(private_key) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        match encrypter.set_rsa_padding(Padding::PKCS1) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(_) => (),
        }

        let buffer_len = match encrypter.encrypt_len(content.as_bytes()) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        let mut encrypted_content = vec![0; buffer_len];

        let encrypted_len = match encrypter.encrypt(content.as_bytes(), &mut encrypted_content) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        encrypted_content.truncate(encrypted_len);

        let encrypted_message_hex = Self::bytes_to_hex_string(encrypted_content);

        Outcome::Success(encrypted_message_hex)
    }

    pub async fn decrypt_content(content: String) -> Outcome<String, CodeMessage, CodeMessage> {
        let private_key = match Self::select_private_key().await {
            Outcome::Error(err) => return Outcome::Error(err),
            Outcome::Failure(fail) => return Outcome::Failure(fail),
            Outcome::Success(val) => val,
        };

        let encrypted_content_vec = match Self::hex_to_vec_u8(&content) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        let mut decrypter = match Decrypter::new(&private_key) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        match decrypter.set_rsa_padding(Padding::PKCS1) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        let buffer_len = match decrypter.decrypt_len(&encrypted_content_vec) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        let mut decrypted_content = vec![0; buffer_len];

        let decrypted_len = match decrypter.decrypt(&encrypted_content_vec, &mut decrypted_content)
        {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        decrypted_content.truncate(decrypted_len);

        let decrypted_string = match String::from_utf8(decrypted_content.to_vec()) {
            Err(err) => {
                return Outcome::Error(CodeMessage {
                    http_code: http::StatusCode::INTERNAL_SERVER_ERROR,
                    message: err.to_string(),
                })
            }
            Ok(val) => val,
        };

        Outcome::Success(decrypted_string)
    }

    fn bytes_to_hex_string(bytes: Vec<u8>) -> String {
        let hex_chars: Vec<String> = bytes.iter().map(|b| format!("{:02x}", b)).collect();
        hex_chars.join("")
    }

    fn hex_to_vec_u8(hex: &str) -> Result<Vec<u8>, String> {
        // Check if the input string has an even length
        if hex.len() % 2 != 0 {
            return Err("Hex string must have an even length".to_string());
        }

        // Collect the byte values by parsing each pair of characters
        (0..hex.len())
            .step_by(2)
            .map(|i| {
                u8::from_str_radix(&hex[i..i + 2], 16)
                    .map_err(|e| format!("Failed to parse hex: {}", e))
            })
            .collect()
    }
}
