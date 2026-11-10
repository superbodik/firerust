use fr_mem::format::format_bytes::format_bytes;
use fr_mem::meter::process_memory::ProcessMemory;

use crate::model::tab::Tab;

pub fn memory_summary(
    process: Option<ProcessMemory>,
    tabs: &[Tab],
    image_cache_bytes: usize,
) -> String {
    let process_text = match process {
        Some(memory) => format_bytes(memory.resident_bytes),
        None => "unknown".to_owned(),
    };
    let tab_texts: Vec<String> = tabs
        .iter()
        .enumerate()
        .map(|(index, tab)| {
            if tab.is_discarded() {
                format!("{}: discarded", index + 1)
            } else {
                format!(
                    "{}: {}",
                    index + 1,
                    format_bytes(tab.footprint_bytes() as u64)
                )
            }
        })
        .collect();
    format!(
        "RAM {process_text} | tabs {} | image cache {}",
        tab_texts.join(", "),
        format_bytes(image_cache_bytes as u64)
    )
}
