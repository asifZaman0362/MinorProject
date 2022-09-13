use aes::Aes256;
use aes::cipher {
    BlockCipher, BlockEncrypt,
    BlockDecrypt, GenericArray,
    typenum::U32
};
use rand::RngCore;

struct Cipher {
    cipher: Aes256,
    key:    [u8; 32]
}

impl Cipher {
    pub fn new(key: [u8; 32]) -> Cipher {
        let key = 
            GenericArray<u8, U32>::from(key.try_into());
        let cipher = Aes256::new(key);
        Cipher {
            cipher,
            key
        }
    }
}
