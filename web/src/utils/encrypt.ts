//https://github.com/travist/jsencrypt

import {JSEncrypt} from "jsencrypt";

const crypt = new JSEncrypt();
const decrpt = new JSEncrypt();

const pub_key_pem = `-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQCxC6org1ozXX9i+FS+eLwM0+kv
00ABH0Nf4TC0Ruisw86gi/tLgTaMFC2kH58aTCAXitXbRnY8TVVxKpo3n193Qami
Fpdv9W3pWX7VgLVxcQ+fG1j8mSDUX1IcsIAHU+ugt51i4/CMc8r43oEAvoHe1Nc7
9JqHpiPMjC9n5jkVewIDAQAB
-----END PUBLIC KEY-----`;

const priv_key_pem = `-----BEGIN PRIVATE KEY-----
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
    -----END PRIVATE KEY-----`;

export function encrypt(plaintext:string) : string {
  // Create the encryption object and set the key.
  const crypt = new JSEncrypt();
  crypt.setPublicKey(pub_key_pem);
  crypt.setPrivateKey(priv_key_pem);
  const text = '123456';
// Encrypt the data with the public key.
  const enc = crypt.encrypt(plaintext);
// Now decrypt the crypted text with the private key.
//   const dec = crypt.decrypt(enc);

// Now a simple check to see if the round-trip worked.
//   if (dec === text){
//     alert('It works!!!');
//   } else {
//     alert('Something went wrong....');
//   }
  return typeof enc === "string" ? enc : ""
}