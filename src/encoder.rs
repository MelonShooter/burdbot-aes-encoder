use aes::cipher::generic_array::GenericArray;
use aes::{Aes256, BlockEncrypt, NewBlockCipher};
use hex::{decode, encode};

use crate::secret;

pub fn aes_encode(str: String) -> String {
    let mut string;
    let mut str_bytes = str.as_bytes();
    let pad_count = 16 - str.len() % 16;

    if pad_count != 0 {
        string = String::with_capacity(pad_count + str_bytes.len());

        for _ in 0..pad_count {
            string.push('0');
        }

        string.push_str(str.as_str());

        str_bytes = string.as_bytes();
    }

    let key = decode(secret::AES_KEY).expect("Bad key.");
    let cipher = Aes256::new_from_slice(key.as_slice()).expect("Bad key.");
    let mut encoded_bytes = String::with_capacity(str_bytes.len() * 2 + 1);

    encoded_bytes.push('f');

    for start in (0..str_bytes.len()).step_by(16) {
        let mut block = *GenericArray::from_slice(&str_bytes[start..start + 16]);

        cipher.encrypt_block(&mut block);

        encoded_bytes.push_str(encode(block).as_str());
    }

    encoded_bytes
}
