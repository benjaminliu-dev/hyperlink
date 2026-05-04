use std::error::Error;
use bitflags::bitflags;
use crc::{CRC_16_MODBUS, Crc};

pub const SYNC_TAG: u16 = 0xA5;

pub enum PACKET_TYPE {
    ACK,       // 0x1
    TELEMETRY, // 0x2
    CMD,       // 0x3
    DBG,       // 0x4
}

bitflags! {
    pub struct Flags: u8 {
        const ACK_REQUIRED = 0b0000_0001;
        const COMPRESSED = 0b0000_0010;
        const ENCRYPTED  = 0b0000_0100;
        const FRAGMENT   = 0b0000_1000;
    }
}

pub struct Packet {
    pub sync: u16, // A sync number to state the start of a new packet
    pub packet_type: u8,
    pub flags: Flags,
    pub sequence: u16,
    pub payload: Vec<u8>,
    pub crc: u16,
}

impl Packet {
    pub fn new(
        ptype: PACKET_TYPE,
        msg_flags: Flags,
        msg_payload_str: &'static str,
        seq: u16,
    ) -> Result<Packet, Box<dyn Error>> {
        let msg_payload = msg_payload_str.as_bytes().to_vec();

        let payload_crc = Crc::<u16>::new(&CRC_16_MODBUS);

        let checksum = payload_crc.checksum(msg_payload.as_slice());

        Ok(Packet {
            sync: SYNC_TAG,
            packet_type: match ptype {
                PACKET_TYPE::ACK => 0x1,
                PACKET_TYPE::TELEMETRY => 0x2,
                PACKET_TYPE::CMD => 0x3,
                PACKET_TYPE::DBG => 0x4,
            },
            flags: msg_flags,
            sequence: seq,
            payload: msg_payload,
            crc: checksum,
        })
    }
}


