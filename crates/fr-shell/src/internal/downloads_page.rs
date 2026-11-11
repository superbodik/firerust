use fr_download::record::download_record::DownloadRecord;
use fr_download::save::downloads_folder::downloads_folder;
use fr_page::load::escape_html::escape_html;

use crate::internal::group_html::group_html;
use crate::internal::page_html::page_html;
use crate::internal::row::Row;

fn size_text(bytes: usize) -> String {
    const KIB: f64 = 1024.0;
    const MIB: f64 = KIB * 1024.0;
    let value = bytes as f64;
    if value >= MIB {
        format!("{:.1} MB", value / MIB)
    } else if value >= KIB {
        format!("{:.0} KB", value / KIB)
    } else {
        format!("{bytes} B")
    }
}

fn download_row(record: &DownloadRecord) -> Row {
    Row::text(&format!(
        "<span class=\"grow\">{}<br><span class=\"note\">{}</span></span>\
         <span class=\"note\">{}</span>",
        escape_html(&record.name),
        escape_html(&record.path.display().to_string()),
        size_text(record.bytes)
    ))
}

pub fn downloads_page(records: &[DownloadRecord], css: &str) -> String {
    let folder = escape_html(&downloads_folder().display().to_string());
    let mut body = format!(
        "<h1>Downloads</h1>\
         <p class=\"lede\">Files saved in this session go to {folder}.</p>"
    );
    if records.is_empty() {
        body.push_str(
            "<div class=\"callout\"><p>No downloads yet.</p>\
             <p>A link to an archive, a PDF or any file the browser cannot show is saved \
             to your Downloads folder.</p></div>",
        );
    } else {
        let rows: Vec<Row> = records.iter().map(download_row).collect();
        body.push_str(&group_html(&rows));
    }
    page_html("Downloads", css, &body)
}
