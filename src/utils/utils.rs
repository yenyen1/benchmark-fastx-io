use std::fs::File;
use std::io::{BufRead, BufReader, Read};

use flate2::read::MultiGzDecoder;

// use std::sync::OnceLock;
// use std::thread;

// static NTHREADS: OnceLock<usize> = OnceLock::new();

// pub fn nthreads() -> usize {
//     *NTHREADS.get_or_init(|| {
//         thread::available_parallelism()
//             .map(|n| n.get())
//             .unwrap_or(1)
//     })
// }

const GZ_MAGIC: [u8; 3] = [0x1F, 0x8B, 0x08];

pub fn open_bufreader(path: &str) -> std::io::Result<Box<dyn BufRead + Send>> {
    if is_gzipped(path)? {
        let decoder = File::open(path).map(MultiGzDecoder::new)?;
        Ok(Box::new(BufReader::new(decoder)))
    } else {
        let file = File::open(path)?;
        Ok(Box::new(BufReader::new(file)))
    }
}

fn is_gzipped(path: &str) -> std::io::Result<bool> {
    let mut magic_bytes: [u8; 3] = [0; 3];
    let mut file = File::open(path)?;
    file.read_exact(&mut magic_bytes)?;

    Ok(magic_bytes == GZ_MAGIC)
}
