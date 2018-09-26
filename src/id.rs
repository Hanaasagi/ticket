use std::str;

#[derive(Debug, Copy, Clone, Default)]
pub struct ID {
    raw: [u8; ::RAW_LEN]
}

impl PartialEq for ID {
    fn eq(&self, other: &ID) -> bool {
        self.raw == other.raw
    }
}

impl ToString for ID {
    fn to_string(&self) -> String {
        str::from_utf8(&self.raw).unwrap().to_string()
    }
}

impl ID {
    pub fn new(raw: [u8; ::RAW_LEN]) -> Self {
        ID {
            raw
        }
    }

    pub fn as_bytes(self) -> [u8; ::RAW_LEN] {
        self.raw
    }
}
