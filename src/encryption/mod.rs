use aes::{Aes128, Block};
use aes::cipher::generic_array::GenericArray;
use aes::cipher::generic_array::typenum::U16;
use aes::cipher::{BlockDecrypt, BlockEncrypt, Key, KeyInit};

pub fn encrypt(key: [u8; 16], content: Vec<[u8; 16]>) -> Vec<u8> {
    let key: GenericArray<_, U16> = GenericArray::clone_from_slice(&key[0..16]);
    let mut cipher = Aes128::new(&key);
    let mut encrypted : Vec<u8> = vec![];
    for block in content {
        let mut aes_block= Block::from(block);
        cipher.encrypt_block(&mut aes_block);
        for byte in aes_block.to_vec() {
            encrypted.push(byte);
        }
    }
    encrypted
}

pub fn decrypt(key: [u8; 16], cipher_text: Vec<[u8; 16]>) -> Vec<u8> {
    let key = GenericArray::clone_from_slice(&key);
    let mut cipher = Aes128::new(&key);
    let mut decrypted : Vec<u8> = vec![];
    for block in cipher_text {
        let mut aes_block = Block::from(block);
        cipher.decrypt_block(&mut aes_block);
        for byte in aes_block {
            decrypted.push(byte);
        }
    }
    decrypted
}