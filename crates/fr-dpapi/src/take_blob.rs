use std::slice;

use crate::data_blob::DataBlob;
use crate::native::LocalFree;

pub fn take_blob(blob: DataBlob) -> Vec<u8> {
    if blob.data.is_null() {
        return Vec::new();
    }
    let bytes = unsafe { slice::from_raw_parts(blob.data, blob.length as usize) }.to_vec();
    unsafe { LocalFree(blob.data.cast()) };
    bytes
}
