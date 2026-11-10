use fr_page::load::page::Page;

pub fn window_title(page: &Page) -> String {
    match &page.title {
        Some(title) => format!("{title} - firerust"),
        None => format!("{} - firerust", page.url),
    }
}
