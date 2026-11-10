use std::sync::Mutex;
use std::thread;

use fr_net::load::load_url::load_url;
use url::Url;

use crate::prefetch::resource_store::ResourceStore;

const WORKERS: usize = 6;

pub fn fetch_in_parallel(urls: Vec<Url>, store: &ResourceStore) {
    let workers = WORKERS.min(urls.len());
    if workers == 0 {
        return;
    }
    let queue = Mutex::new(urls);
    thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let next = match queue.lock() {
                        Ok(mut waiting) => waiting.pop(),
                        Err(_) => None,
                    };
                    let Some(url) = next else {
                        break;
                    };
                    if let Ok(response) = load_url(&url) {
                        store.insert(&url, response);
                    }
                }
            });
        }
    });
}
