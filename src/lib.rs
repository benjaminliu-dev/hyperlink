pub mod communication;
pub mod hlcp;
#[cfg(test)]
mod tests {
    use crate::communication::{Transport, send_packet};
    use crate::hlcp::{Flags, PacketType, Packet};
    use std::error::Error;
    struct TestTransportSend {}
    impl Transport for TestTransportSend {
        fn open_line(&mut self) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn close_line(&mut self) -> Result<(), Box<dyn Error>> {
            Ok(())
        }

        fn send_byte(&mut self, data: u8) -> Result<(), Box<dyn Error>> {
            print!("{:X}", data);
            Ok(())
        }
    }
    #[test]
    fn test_hlcp_packet() {
        let pckt: Packet = Packet::new(
            PacketType::TELEMETRY,
            Flags::empty() | Flags::ACK_REQUIRED | Flags::ENCRYPTED,
            "Hello world",
            0,
        )
        .unwrap();
        println!(
            "{:X}, {:X}, {:08b}, {:X}",
            pckt.sync, pckt.packet_type, pckt.flags, pckt.crc
        );
    }

    #[test]
    fn test_send_packet() {
        let mut transport: TestTransportSend = TestTransportSend {};
        let flags: Flags = Flags::empty();
        send_packet(
            Packet::new(
                PacketType::TELEMETRY,
                flags | Flags::ENCRYPTED,
                "Ben da ben",
                1,
            )
            .unwrap(),
            &mut transport,
        )
        .unwrap();
    }
}
