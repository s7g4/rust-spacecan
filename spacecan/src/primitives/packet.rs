use crate::PacketData;

#[derive(Debug, Clone, PartialEq)]
pub struct SpaceCANPacket {
    pub version: u8,
    pub packet_type: u8,
    pub data_field_header_flag: bool,
    pub application_process_id: u16,
    pub sequence_flags: u8,
    pub sequence_count: u16,
    pub data_length: u16,
    pub data: PacketData,
}

impl SpaceCANPacket {
    pub fn new(
        application_process_id: u16,
        sequence_count: u16,
        data: PacketData,
    ) -> Result<Self, &'static str> {
        let len: u16 = data.len().try_into().map_err(|_| "Data too long")?;
        Ok(SpaceCANPacket {
            version: 0,
            packet_type: 0,
            data_field_header_flag: false,
            application_process_id,
            sequence_flags: 3,
            sequence_count,
            data_length: len,
            data,
        })
    }
}
