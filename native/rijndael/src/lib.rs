use simple_rijndael::impls::RijndaelCbc;
use simple_rijndael::paddings::Pkcs7Padding;
use base64::prelude::*;
use pbkdf2::pbkdf2_array;
use hmac::Hmac;
use sha1::Sha1;

#[rustler::nif]
fn decrypt(ciphertext: String, pass_phrase: String) -> String {
    let keysize: usize = 256; // Key size in bits

    let cipher = BASE64_STANDARD.decode(ciphertext).unwrap().to_vec();

    let salt_bytes = &cipher[0..keysize / 8];
    let iv_bytes = &cipher[keysize / 8..2 * keysize / 8];
    let ciphertext_bytes = &cipher[2 * keysize / 8..].to_vec();

    let key = pbkdf2_array::<Hmac<Sha1>, 32>(pass_phrase.as_bytes(), salt_bytes, 1000).unwrap();

    let bytes = RijndaelCbc::<Pkcs7Padding>::new(&key, 32)
        .unwrap()
        .decrypt(&iv_bytes, ciphertext_bytes.clone())
        .unwrap();

    match std::str::from_utf8(&bytes) {
        Ok(v) => v.to_string(),
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    }
}

rustler::init!("Elixir.Rijndael", [decrypt]);
