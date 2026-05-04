# hyperlink

A 2-way synchronous communications library for aerospace & robotics

Parts:
- HLCP: Hyper-Link Communication Protocol is structured
`sync: u16,
    packet_type: u8,
    flags: Flags (u8),
    sequence: u16
    payload: Vec<u8>,
    crc: u16`
- Transports: Customizable comms methods
- Communications: communications

Notes:
- CRC checksums are CRC 16 MODBUS


Changelog:
- v0.1.0: Basic implementation; synchronous. 