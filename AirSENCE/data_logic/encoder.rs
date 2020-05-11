use mqtt::{Encodable};
use mqtt::packet::{PublishPacket};

/*** 
 * Encode a packet and return the encoded Vector 
 * Note: Input needs to be cloned
 * ***/
 pub fn encode(packet: PublishPacket) -> Vec<u8> {  
    let mut buffer = Vec::new();
    packet.encode(&mut buffer).unwrap();
    return buffer;
}