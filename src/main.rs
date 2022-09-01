use nom::AsBytes;
use pbkdf2::{
    password_hash::{
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Pbkdf2
};
use pbkdf2::password_hash::rand_core;
use rand::rngs::OsRng;

mod encryption;

fn encrypt(key: &str, string: &str) -> Vec<u8> {
    let ciphertext: Vec<u8>;
    let mut bytes = string.bytes();
    let mut i: usize = 0;
    let mut blocks: Vec<[u8; 16]> = vec![];
    let mut block: [u8; 16] = [0; 16];
    loop {
        if i % 16 == 0 && i > 0 {
            blocks.push(block.clone());
            block = [0; 16];
        }
        match bytes.next() {
            Some(byte) => {
                block[i % 16] = byte;
                i += 1;
            },
            None => break
        };
    }
    if i % 16 != 0 {
        blocks.push(block.clone());
    }
    let salt = pbkdf2::password_hash::SaltString::generate(&mut OsRng);
    let key_hash = match pbkdf2::Pbkdf2.hash_password(key.as_bytes(), &salt) {
        Ok(hash) => hash.to_string().as_bytes(),
        Err(error) => panic!("failed to hash password")
    };
    encryption::encrypt(key_hash.clone_from_slice(), blocks)
}

fn main() {
    let mut input: String = "".to_owned();
    let ciphertext: Vec<u8>;
    let stdin = std::io::stdin();
    match stdin.read_line(&mut input) {
        Ok(bytes) => ciphertext = encrypt(input),
        Err(error) => panic!("failed to read from stdin")
    }

}