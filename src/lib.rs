use bencode::bencode::{BencodeElement, BencodeError};
use std::collections::{BTreeMap, HashMap};

#[derive(Debug)]
pub struct TorrentInfo {
    length: usize,
    name: String,
    piece_length: usize,
    pieces: Vec<u8>,
}

#[derive(Debug)]
pub struct Torrent {
    announce: String,
    info: TorrentInfo,
}

impl Torrent {
    pub fn new(bencode_dict: BencodeElement) -> Result<Self, BencodeError> {
        match <BencodeElement as TryInto<BTreeMap<String, BencodeElement>>>::try_into(bencode_dict)
        {
            Ok(x) => {
                let info_dict: BTreeMap<String, BencodeElement> =
                    x.get("info").unwrap().clone().try_into().unwrap();

                let info = TorrentInfo {
                    length: <BencodeElement as TryInto<i64>>::try_into(
                        info_dict.get("length").unwrap().clone(),
                    )
                    .unwrap() as usize,
                    name: info_dict.get("name").unwrap().clone().try_into().unwrap(),
                    piece_length: <BencodeElement as TryInto<i64>>::try_into(
                        info_dict.get("piece length").unwrap().clone(),
                    )
                    .unwrap() as usize,
                    pieces: info_dict.get("pieces").unwrap().clone().try_into().unwrap(),
                };
                let torrent = Torrent {
                    announce: x.get("announce").unwrap().clone().try_into().unwrap(),
                    info: info,
                };
                Ok(torrent)
            }
            Err(_) => Err(BencodeError::InvalidElementError(
                "Given Bencoded Element is not Dict".to_owned(),
                "".to_owned(),
            )),
        }
    }
}
