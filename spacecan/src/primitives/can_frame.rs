use crate::FrameData;

#[derive(Debug, Clone, PartialEq)]
pub struct CanFrame {
    id: u32,
    data: FrameData,
}

#[derive(Debug)]
pub enum CanFrameError {
    InvalidId,
    DataTooLong,
    SendFailed,
}

impl CanFrame {
    pub fn new(id: u32, data: Option<FrameData>) -> Result<Self, CanFrameError> {
        let frame_data = data.unwrap_or_else(|| FrameData::new());
        if frame_data.len() > 8 {
            return Err(CanFrameError::DataTooLong);
        }
        Ok(CanFrame {
            id,
            data: frame_data,
        })
    }

    pub fn can_id(&self) -> u32 {
        self.id
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
