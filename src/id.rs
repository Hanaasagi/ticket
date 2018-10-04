/// ticket id representation
#[derive(Debug, Copy, Clone, Default)]
pub struct ID {
    raw: [u8; ::RAW_LEN]
}

impl PartialEq for ID {
    fn eq(&self, other: &ID) -> bool {
        self.raw == other.raw
    }
}


impl ID {
    pub fn new(raw: [u8; ::RAW_LEN]) -> Self {
        ID {
            raw
        }
    }

    /// covert ID to u8 array
    pub fn as_bytes(self) -> [u8; ::RAW_LEN] {
        self.raw
    }
}
