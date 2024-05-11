use bencode::bencode::decode_bencode_element;
use std::{env, fs, io};
use wolf_torrent::Torrent;

fn read_torrent_file(filepath: &str) -> Result<Vec<u8>, io::Error> {
    fs::read(filepath)
}

fn main() {
    let filepath = env::args().nth(1).unwrap();
    let file_bytes = read_torrent_file(&filepath).unwrap();
    let bencode_dict = decode_bencode_element(file_bytes).unwrap();
    let t = Torrent::new(bencode_dict).unwrap();
    println!("{:?}", t)
}
