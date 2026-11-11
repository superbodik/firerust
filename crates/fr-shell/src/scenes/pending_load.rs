use std::sync::mpsc::{Receiver, TryRecvError, channel};
use std::thread;
use std::time::Instant;

use fr_net::load::request::Request;
use fr_page::prefetch::fetched::Fetched;
use fr_page::prefetch::prefetch_request::prefetch_request;
use fr_tab::model::tab_id::TabId;
use url::Url;

use crate::scenes::load_target::LoadTarget;

const WORKER_STACK_BYTES: usize = 16 * 1024 * 1024;

pub struct PendingLoad {
    receiver: Receiver<Result<Fetched, String>>,
    pub target: LoadTarget,
    pub tab: TabId,
    pub url: Url,
    pub started: Instant,
}

impl PendingLoad {
    pub fn start(request: Request, target: LoadTarget, tab: TabId) -> Self {
        let (sender, receiver) = channel();
        let url = request.url.clone();
        let spawned = thread::Builder::new()
            .stack_size(WORKER_STACK_BYTES)
            .spawn(move || {
                let outcome = prefetch_request(&request).map_err(|error| error.to_string());
                let _ = sender.send(outcome);
            });
        if let Err(error) = spawned {
            eprintln!("cannot start the loading thread: {error}");
        }
        Self {
            receiver,
            target,
            tab,
            url,
            started: Instant::now(),
        }
    }

    pub fn poll(&self) -> Option<Result<Fetched, String>> {
        match self.receiver.try_recv() {
            Ok(outcome) => Some(outcome),
            Err(TryRecvError::Empty) => None,
            Err(TryRecvError::Disconnected) => Some(Err("loading was interrupted".to_owned())),
        }
    }
}
