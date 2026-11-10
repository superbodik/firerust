use std::sync::OnceLock;
use std::time::Duration;

use ureq::Agent;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

pub fn shared_agent() -> &'static Agent {
    static AGENT: OnceLock<Agent> = OnceLock::new();
    AGENT.get_or_init(|| {
        Agent::config_builder()
            .timeout_global(Some(REQUEST_TIMEOUT))
            .http_status_as_error(false)
            .build()
            .into()
    })
}
