use std::ptr;

#[repr(C)]
pub struct DataBlob {
    pub length: u32,
    pub data: *mut u8,
}

impl DataBlob {
    pub fn empty() -> Self {
        Self {
            length: 0,
            data: ptr::null_mut(),
        }
    }

    pub fn borrowing(bytes: &[u8]) -> Option<Self> {
        let length = u32::try_from(bytes.len()).ok()?;
        Some(Self {
            length,
            data: bytes.as_ptr().cast_mut(),
        })
    }
}
