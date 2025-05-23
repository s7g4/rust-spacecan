use embedded_hal::can::{Frame as EmbeddedFrame};
use bxcan::{Frame, Id, StandardId, ExtendedId, Data};

/// Represents a SpaceCAN frame with a command ID and payload.
pub struct SpaceCANFrame {
    pub command_id: u16,
    pub payload: [u8; 8],
    pub payload_len: usize,
}

impl SpaceCANFrame {
    /// Creates a new SpaceCAN frame.
    pub fn new(command_id: u16, payload: &[u8]) -> Result<Self, SpaceCANError> {
        if payload.len() > 8 {
            return Err(SpaceCANError::PayloadTooLarge);
        }
        let mut frame_payload = [0u8; 8];
        frame_payload[..payload.len()].copy_from_slice(payload);
        Ok(SpaceCANFrame {
            command_id,
            payload: frame_payload,
            payload_len: payload.len(),
        })
    }
}

/// Custom error type for SpaceCAN.
#[derive(Debug)]
pub enum SpaceCANError {
    PayloadTooLarge,
    NoFrameReceived,
    WouldBlock,
    Other,
}

/// Represents a SpaceCAN interface for sending and receiving frames.
pub struct SpaceCAN<CAN> {
    can: CAN,
}

impl<CAN> SpaceCAN<CAN>
where
    CAN: embedded_hal::can::nb::Can<Frame = bxcan::Frame>,
{
    /// Creates a new SpaceCAN interface.
    pub fn new(can: CAN) -> Self {
        SpaceCAN { can }
    }

    /// Sends a SpaceCAN frame.
    pub fn send_frame(&mut self, frame: &SpaceCANFrame) -> Result<(), nb::Error<CAN::Error>> {
        let id = if frame.command_id <= 0x7FF {
            bxcan::Id::Standard(bxcan::StandardId::new(frame.command_id).unwrap())
        } else {
            bxcan::Id::Extended(bxcan::ExtendedId::new(frame.command_id as u32).unwrap())
        };

        let data = bxcan::Data::new(&frame.payload[..frame.payload_len]).unwrap();
        let can_frame = Frame::new_data(id, data);

        match self.can.transmit(&can_frame) {
            Ok(Some(_)) => Ok(()),
            Ok(None) => Err(nb::Error::WouldBlock),
            Err(e) => Err(e),
        }
    }

    /// Receives a SpaceCAN frame.
    pub fn receive_frame(&mut self) -> Result<SpaceCANFrame, nb::Error<CAN::Error>> {
        let frame = self.can.receive()?;
        let command_id = match frame.id() {
            bxcan::Id::Standard(id) => id.as_raw() as u16,
            bxcan::Id::Extended(id) => id.as_raw() as u16,
        };
        let payload = frame.data().ok_or(nb::Error::WouldBlock)?; // Handle None case
        SpaceCANFrame::new(command_id, payload).map_err(|_| nb::Error::WouldBlock)
    }
}