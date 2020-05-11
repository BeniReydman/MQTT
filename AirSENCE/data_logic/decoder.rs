use std::io::Cursor;

use mqtt::{Decodable};
use mqtt::packet::{VariablePacket, PublishPacket};

/*** Decode an encoded Vector with known type and return the PublishedPacket ***/
pub fn decode(buffer: &Vec<u8>) -> PublishPacket {
    let mut dec_buf = Cursor::new(&buffer[..]);
    let decoded = PublishPacket::decode(&mut dec_buf).unwrap();
    return decoded;
}

/*** Auto decode an encoded Vector by fixed header and return the VariablePacket ***/
pub fn auto_decode(buffer: &Vec<u8>) -> VariablePacket {
    let mut dec_buf = Cursor::new(&buffer[..]);
    let auto_decoded = VariablePacket::decode(&mut dec_buf).unwrap();
    return auto_decoded;
}