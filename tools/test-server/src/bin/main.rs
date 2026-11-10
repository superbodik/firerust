use fr_test_server::server::test_server::TestServer;

fn main() -> std::process::ExitCode {
    let address = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_owned());
    match TestServer::start_on(&address) {
        Ok(server) => {
            eprintln!("test server on http://{}", server.address());
            server.wait();
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("cannot start the test server: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
