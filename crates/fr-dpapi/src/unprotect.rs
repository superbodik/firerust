#[cfg(windows)]
pub fn unprotect(data: &[u8], entropy: &[u8]) -> Option<Vec<u8>> {
    use std::ptr;

    use crate::data_blob::DataBlob;
    use crate::native::CryptUnprotectData;
    use crate::take_blob::take_blob;

    const UI_FORBIDDEN: u32 = 0x1;
    let input = DataBlob::borrowing(data)?;
    let extra = DataBlob::borrowing(entropy)?;
    let mut output = DataBlob::empty();
    let succeeded = unsafe {
        CryptUnprotectData(
            &input,
            ptr::null_mut(),
            &extra,
            ptr::null(),
            ptr::null(),
            UI_FORBIDDEN,
            &mut output,
        )
    };
    if succeeded == 0 {
        return None;
    }
    Some(take_blob(output))
}

#[cfg(not(windows))]
pub fn unprotect(_data: &[u8], _entropy: &[u8]) -> Option<Vec<u8>> {
    None
}
