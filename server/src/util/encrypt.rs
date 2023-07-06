use anyhow::bail;
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use rsa::pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding};
use base64::{decode, encode};
use rand::prelude::ThreadRng;

const KEY_SIZE: usize = 1024;

pub fn generate_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let mut rng = rand::thread_rng();
    let priv_key = RsaPrivateKey::new(&mut rng, KEY_SIZE).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);
    (priv_key, pub_key)
}

pub fn fmt_print_keys(pub_key: &RsaPublicKey, priv_key: &RsaPrivateKey) -> (String, String) {
    let pub_key_pem = pub_key.to_public_key_pem(LineEnding::LF).expect("failed to convert public key to PKCS#8 PEM");
    println!("Public Key (PKCS#8 PEM): \n{}", pub_key_pem.as_str());
    let priv_key_pem = priv_key.to_pkcs8_pem(LineEnding::LF).expect("failed to convert private key to PKCS#8 PEM");
    println!("Private Key (PKCS#8 PEM): \n{}", priv_key_pem.as_str());
    (pub_key_pem.as_str().to_string(), priv_key_pem.as_str().to_string())
}

// 加密
pub fn encrypt(pub_key: &RsaPublicKey, ciphertext: &[u8]) -> String {
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &ciphertext[..]).expect("failed to encrypt");
    encode(&enc_data)
}

// 解密
pub fn decrypt(priv_key: &RsaPrivateKey, ciphertext: &[u8]) -> String {
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, ciphertext).expect("failed to decrypt");
    std::str::from_utf8(&dec_data).unwrap().to_string()
}

// 创建一个全局的公私钥
lazy_static::lazy_static! {
    pub static ref DEFAULT_KEY: (RsaPrivateKey, RsaPublicKey) = {
        default_keys()
    };
}

// 缺省秘钥
pub fn default_keys() -> (RsaPrivateKey, RsaPublicKey) {
    let pub_key_pem = "-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCxC6org1ozXX9i+FS+eLwM0+kv
00ABH0Nf4TC0Ruisw86gi/tLgTaMFC2kH58aTCAXitXbRnY8TVVxKpo3n193Qami
Fpdv9W3pWX7VgLVxcQ+fG1j8mSDUX1IcsIAHU+ugt51i4/CMc8r43oEAvoHe1Nc7
9JqHpiPMjC9n5jkVewIDAQAB
-----END PUBLIC KEY-----";
    let priv_key_pem = "-----BEGIN PRIVATE KEY-----
MIICdwIBADANBgkqhkiG9w0BAQEFAASCAmEwggJdAgEAAoGBALELqiuDWjNdf2L4
VL54vAzT6S/TQAEfQ1/hMLRG6KzDzqCL+0uBNowULaQfnxpMIBeK1dtGdjxNVXEq
mjefX3dBqaIWl2/1belZftWAtXFxD58bWPyZINRfUhywgAdT66C3nWLj8Ixzyvje
gQC+gd7U1zv0moemI8yML2fmORV7AgMBAAECgYEAkwbl3dbqXIfD11P1RGyNdnWx
TEc5+vSUhhZD1SKh7X96o/c7zHtU6kWDla2w+izH5eUlLkE4xWFPmHhRs+mFY+iR
QkRsdS5+AZ0o/xZoOzN+Wk+m7yH8iWJA/I8HIKnUVci76Yjf/+PKDhfXxfbFDaWz
msnfWBqiM2Qu2xGmtkECQQDEydSq0rtE28DeiKH4T1lFTGFMlmsyOspFPTwU1wR3
JdVCX04LSJUyYcsy8vTKK6Wf730jE8ELGklFsjGWYvihAkEA5lEWbc07FuFz9Jel
XGdFrkT9Yq516LkL7/QKpSdeYgUHWG3PDubQOGdK8lKulA0zcOjQxmf/3TZeCyjw
aYUMmwJBALAy5zBsrzDgcrjOLfhfU9cwpI4mwKSg7ys/wOQIAf+M6H4dR4vi2XPb
DE36LCQ5IHsrvCgEL/z49FJrUusMgwECQH3Zwzq+kVJAYZ8+mgIzADVox1h74jwH
mYlFvede6sj+3HOqlSEcaJzZfG+LVI/b4scIwMQZogtboOnSNgRCyo8CQBd8PS6F
VcJC60FS26OgHgAixikmIsdR5JCSO3PmLVtRbGsHmIZm9NR4YV8U/X3NRw+ZBQ1Z
27pOPx4/O7yT3yY=
-----END PRIVATE KEY-----";
    //从上面的字符串读取公钥和私钥
    let pub_key = RsaPublicKey::from_public_key_pem(pub_key_pem).unwrap();
    let priv_key: RsaPrivateKey =  DecodePrivateKey::from_pkcs8_pem(priv_key_pem).unwrap();
    (priv_key, pub_key)
}

// 缺省加密
pub fn default_encrypt(ciphertext: &[u8]) -> Result<String,anyhow::Error> {
    let (_, pub_key) = &*DEFAULT_KEY;
    let mut rng = rand::thread_rng();
    let enc_data = pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &ciphertext[..])?;
    Ok(encode(&enc_data))
}

// 缺省解密
pub fn default_decrypt(ciphertext: &[u8]) -> Result<String,anyhow::Error>  {
    let (priv_key,_) = &*DEFAULT_KEY;
    let dec_data = priv_key.decrypt(Pkcs1v15Encrypt, ciphertext)?;;
    Ok(std::str::from_utf8(&dec_data).unwrap().to_string())
}