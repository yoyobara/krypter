use std::{fs::File, io::Cursor};

use krypter::krypt_encrypt;

fn main() {
    krypt_encrypt(Cursor::new(vec![1, 2, 3, 4, 5]), 2, |i| File::create(format!("out{}.png", i)).unwrap()).unwrap();
}
