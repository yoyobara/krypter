use std::{fs::File, io::BufReader};

use krypter::{krypt_decrypt, krypt_encrypt};

fn main() {
    krypt_encrypt(File::open("./mylargefile").unwrap(), 400, |i| File::create(format!("out{}.png", i)).unwrap()).unwrap();
    
    let readers = vec![BufReader::new(File::open("out0.png").unwrap()), BufReader::new(File::open("out1.png").unwrap()), BufReader::new(File::open("out2.png").unwrap())];
    krypt_decrypt(readers, File::create("wow.txt").unwrap()).unwrap();
}
