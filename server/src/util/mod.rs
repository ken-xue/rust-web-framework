// use aes::Aes128;
// use aes::cipher::{Block, BlockDecrypt, BlockEncrypt, Key, KeyInit};
use uuid::Uuid;
pub mod util;
pub mod encrypt;

pub fn uuid() -> String {
    let uuid = Uuid::new_v4();
    let uuid_string = uuid.to_string().replace("-", "");
    return uuid_string;
}

// fn aes_encrypt(key: &[u8], plaintext: &[u8]) -> Vec<u8> {
//     let cipher = Aes128::new(<&Key<Aes128>>::from(key));
//     let mut ciphertext = plaintext.to_owned();
//     let blocks = ciphertext.chunks_exact_mut(16);
//     for block in blocks {
//         cipher.encrypt_block(<&mut Block<Aes128>>::from(block));
//     }
//     ciphertext
// }
//
// fn aes_decrypt(key: &[u8], ciphertext: &[u8]) -> Vec<u8> {
//     let cipher = Aes128::new(<&Key<Aes128>>::from(key));
//     let mut plaintext = ciphertext.to_owned();
//     let blocks = plaintext.chunks_exact_mut(16);
//     for block in blocks {
//         cipher.decrypt_block(<&mut Block<Aes128>>::from(block));
//     }
//     plaintext
// }

// use rsa::{PublicKey, RSAPrivateKey, RSAPublicKey};
// use rand::rngs::OsRng;

#[cfg(test)]
mod tests {
    // use rand::RngCore;
    // use rand::rngs::OsRng;
    // use super::*;

    #[test]
    fn rsa() {
        use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
        use base64::encode;
        let mut rng = rand::thread_rng();
        let bits = 1024;
        let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);
        println!("priv_key: {:?}", priv_key);
        println!("pub_key: {:?}", pub_key);
// Encrypt
        let data = b"hello world";
        let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
        println!("Encrypted data: {}", encode(&enc_data));
        assert_ne!(&data[..], &enc_data[..]);

// Decrypt
        let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
        assert_eq!(&data[..], &dec_data[..]);
        println!("Decrypted data: {:?}", std::str::from_utf8(&dec_data).unwrap());
    }

}
