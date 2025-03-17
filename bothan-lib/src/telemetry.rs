use std::error::Error;
use std::net::{ToSocketAddrs, SocketAddr};
use std::sync::Arc;

use once_cell::sync::OnceCell;
use opentelemetry_sdk::metrics::MetricError;
use tokio::task::JoinHandle;
use tracing::{debug, warn};

mod server;
mod state;

static GLOBAL_STATE: OnceCell<Arc<state::TelemetryState>> = OnceCell::new();

pub fn init(
) -> Result<&'static Arc<state::TelemetryState>, MetricError> {
    let new_state = state::TelemetryState::build()?;

    match GLOBAL_STATE.set(Arc::new(new_state)) {
        Ok(_) => debug!("initialised telemetry global state"),
        Err(_) => debug!("telemetry global state was already set"),
    }
    
    Ok(GLOBAL_STATE.get().unwrap())
}

pub fn global() -> &'static Arc<state::TelemetryState> {
    match GLOBAL_STATE.get() {
        Some(state) => state,
        None => {
            warn!(
                "global telemetry state not set, attempting to initialize now"
            );
            init().unwrap()
        }
    }
}

pub type BoxError = Box<dyn Error + Send + Sync>;

pub fn spawn_server<A>(
    addr: A,
    state: Arc<state::TelemetryState>,
) -> Result<(SocketAddr, JoinHandle<Result<(), BoxError>>), BoxError>
where
    A: ToSocketAddrs + Send + 'static,
{
    let addr = addr.to_socket_addrs()?.next().unwrap();
    let handle = tokio::spawn(server::listen(addr, state));

    Ok((addr, handle))
}
