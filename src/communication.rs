use std::error::Error;

use crate::hlcp::Packet;

pub trait Transport {
    fn open_line(&mut self) -> Result<(), Box<dyn Error>>;
    fn close_line(&mut self) -> Result<(), Box<dyn Error>>;
    fn send_byte(&mut self, data: u8) -> Result<(), Box<dyn Error>>;
    // fn recieve_byte(&mut self, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>;
}
pub fn send_packet(packet: Packet, transport: &mut dyn Transport) -> Result<(), Box<dyn Error>> {
    // Send data byte by byte
    for byte in packet.sync.to_le_bytes() {
        transport.send_byte(byte)?;
    }

    transport.send_byte(packet.packet_type)?;

    transport.send_byte(packet.flags.bits())?;

    for byte in packet.sequence.to_le_bytes() {
        transport.send_byte(byte)?;
    }

    for byte in packet.crc.to_le_bytes() {
        transport.send_byte(byte)?;
    }

    for byte in &packet.payload {
        transport.send_byte(*byte)?;
    }

    Ok(())
}

pub struct Connection {}
