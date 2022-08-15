use rand::prelude::*;
use std::error::Error;
use std::fs;
use std::io::{Read, Write};
use std::marker;
use std::path;
use zeroize::Zeroize;

pub const DEFAULT_CHUNK_LEN: usize = 0x10;
pub const DEFAULT_BLOCK_LEN: usize = 0x8;
pub const DEFAULT_BUF_LEN: usize = 0xFFFF;

type DataResult<T> = Result<T, Box<dyn Error>>;

pub trait SecretData {}

// Data states.
#[derive(Debug)]
pub enum EncryptedData {}
#[derive(Debug)]
pub enum DecryptedData {}
#[derive(Debug)]
pub enum InvalidData {}

impl SecretData for EncryptedData {}

impl SecretData for DecryptedData {}

impl SecretData for InvalidData {}

#[derive(Debug)]
pub struct DataState<S: SecretData> {
    inner: Option<Vec<u8>>,
    _phantom: marker::PhantomData<S>,
}

fn check_buffer_validity(buf: &[u8], block: usize) -> bool {
    let len = buf.len();

    len % DEFAULT_CHUNK_LEN == 0 && len / DEFAULT_CHUNK_LEN == block
}

fn do_xor_bytes(mut inner: Option<Vec<u8>>, key: &[u8]) -> DataResult<Option<Vec<u8>>> {
    if !check_buffer_validity(key, DEFAULT_BLOCK_LEN) {
        return Err("[-] The key is invalid!".into());
    }

    let size = DEFAULT_BLOCK_LEN * DEFAULT_CHUNK_LEN;

    let mut buf = match inner.take() {
        Some(v) => v,
        None => return Err("[-] The encrypted / decrypted data contains nothing!".into()),
    };

    for i in 0..size {
        buf[i] = key[i] ^ buf[i];
    }

    Ok(Some(buf))
}

impl DataState<InvalidData> {
    pub fn fill_encrypted_bytes(
        self,
        encrypted_data_buffer: &[u8],
    ) -> DataResult<DataState<EncryptedData>> {
        if !check_buffer_validity(encrypted_data_buffer, DEFAULT_BLOCK_LEN) {
            return Err("[-] The encrypted buffer is not valid!".into());
        }

        Ok(DataState {
            inner: Some(encrypted_data_buffer.to_vec()),
            _phantom: marker::PhantomData,
        })
    }

    pub fn ready(self) -> DataResult<DataState<EncryptedData>> {
        match self.inner {
            Some(_) => Ok(DataState {
                inner: self.inner,
                _phantom: marker::PhantomData,
            }),

            None => Err("[-] Cannot convert none data to encrypted state!
                        You must invoke `fill_encrypted_bytes` first."
                .into()),
        }
    }

    pub fn from_file(file_path: &str) -> DataResult<DataState<EncryptedData>> {
        let mut file = fs::File::open(file_path)?;
        let mut buf = vec![0u8; DEFAULT_BUF_LEN];

        file.read_to_end(&mut buf)?;

        Ok(DataState {
            inner: Some(buf),
            _phantom: marker::PhantomData,
        })
    }

    pub fn new(should_fill_random_data: bool) -> DataResult<DataState<InvalidData>> {
        let mut inner: Option<Vec<u8>> = None;
        if should_fill_random_data {
            let mut rng = thread_rng();
            let mut bytes = [0u8; DEFAULT_BLOCK_LEN * DEFAULT_CHUNK_LEN];
            rng.fill_bytes(&mut bytes);
            inner = Some(bytes.to_vec());
        }

        Ok(DataState {
            inner,
            _phantom: marker::PhantomData,
        })
    }
}

impl DataState<EncryptedData> {
    pub fn decrypt(self, key: &[u8]) -> DataResult<DataState<DecryptedData>> {
        Ok(DataState {
            inner: do_xor_bytes(self.inner, key)?,
            _phantom: marker::PhantomData,
        })
    }

    pub fn save_to_file(&self, file_path: &str) -> DataResult<()> {
        // First check if file exists.
        let mut file: fs::File;
        if path::Path::new(file_path).exists() {
            file = fs::File::open(file_path).unwrap();
        } else {
            file = fs::File::create(file_path)?;
        }

        let size = file.write(&self.inner.as_ref().unwrap().as_slice())?;
        println!("[+] Successfully write 0x{:x} bytes to the disk.", size);

        Ok(())
    }
}

impl DataState<DecryptedData> {
    pub fn encrypt(self, key: &[u8]) -> DataResult<DataState<EncryptedData>> {
        Ok(DataState {
            inner: do_xor_bytes(self.inner, key)?,
            _phantom: marker::PhantomData,
        })
    }
}

impl<S> Zeroize for DataState<S>
where
    S: SecretData,
{
    fn zeroize(&mut self) {
        println!("[-] Zeroizing secrets...");
        self.inner.zeroize();
    }
}
