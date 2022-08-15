#![allow(unused_variables, dead_code, non_snake_case)]
extern crate rand;
extern crate zeroize;

mod typestate;

fn main() {
    let buf = [0u8; typestate::DEFAULT_BLOCK_LEN * typestate::DEFAULT_CHUNK_LEN];
    let key = [0x90u8; typestate::DEFAULT_BLOCK_LEN * typestate::DEFAULT_CHUNK_LEN];
    let data = typestate::DataState::<typestate::InvalidData>::new(true).unwrap();
    println!("[+] Encrypted data: {:02x?}", data);
    let encrypted_data = data.ready().expect("OKOK");
    encrypted_data.save_to_file("./data.bin").expect("OKOK");
    let decrypted_data = encrypted_data.decrypt(&key).expect("OKOK");
    println!("[+] Decrypted data: {:02x?}", decrypted_data);
}
