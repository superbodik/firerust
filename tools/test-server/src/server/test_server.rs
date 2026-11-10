use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::{self, JoinHandle};

use crate::server::serve_connection::serve_connection;

pub struct TestServer {
    address: SocketAddr,
    running: Arc<AtomicBool>,
    worker: Option<JoinHandle<()>>,
}

impl TestServer {
    pub fn start() -> std::io::Result<Self> {
        Self::start_on("127.0.0.1:0")
    }

    pub fn start_on(address: &str) -> std::io::Result<Self> {
        let listener = TcpListener::bind(address)?;
        let address = listener.local_addr()?;
        let running = Arc::new(AtomicBool::new(true));
        let flag = running.clone();
        let worker = thread::spawn(move || {
            for stream in listener.incoming() {
                if !flag.load(Ordering::SeqCst) {
                    break;
                }
                if let Ok(stream) = stream {
                    thread::spawn(move || serve_connection(stream));
                }
            }
        });
        Ok(Self {
            address,
            running,
            worker: Some(worker),
        })
    }

    pub fn address(&self) -> SocketAddr {
        self.address
    }

    pub fn url(&self, path: &str) -> String {
        format!("http://{}{}", self.address, path)
    }

    pub fn wait(mut self) {
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}

impl Drop for TestServer {
    fn drop(&mut self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = TcpStream::connect(self.address);
        if let Some(worker) = self.worker.take() {
            let _ = worker.join();
        }
    }
}
