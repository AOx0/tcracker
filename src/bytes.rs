use std::fs::File;
use std::io::{Read, Write};

pub struct Bytes ( pub Vec<u8>, String);

impl Bytes {
    pub fn get_bytes(binary: &str) -> Bytes {
        let mut f = File::open(&binary).expect("no file found");
        let metadata = std::fs::metadata(&binary).expect("unable to read metadata");
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer).expect("buffer overflow");

        Bytes ( buffer, binary.to_string() )
    }

    pub fn get_f_address(&self) -> Option<usize> {
        let pat: [u8; 10] = [0x00, 0x55, 0x48, 0x89, 0xE5, 0x48, 0x8B, 0x7F, 0x18, 0x48];

        self.0.windows(pat.len()).position(|w| w.eq(&pat))
    }

    pub fn write(&self) {
        let mut f = std::fs::OpenOptions::new().write(true).truncate(true).open(&self.1).unwrap();
        f.write_all(&self.0).unwrap();
        f.flush().unwrap();
    }
}


