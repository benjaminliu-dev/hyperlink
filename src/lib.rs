pub mod hlcp;
pub mod communication;
#[cfg(test)]
mod tests {

    use crate::hlcp::{Flags, PACKET_TYPE, Packet};

    #[test]
    fn test_hlcp_packet() {
        let pckt: Packet = Packet::new(PACKET_TYPE::TELEMETRY, Flags::empty() | Flags::ACK_REQUIRED | Flags::ENCRYPTED, "Hello world", 0).unwrap();
        println!("{:X}, {:X}, {:08b}, {:X}", pckt.sync, pckt.packet_type, pckt.flags, pckt.crc);
    }
    
    #[test]
    fn test_send_packet() {
        // to be implemented
    }
}
