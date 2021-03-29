#![feature(map_into_keys_values)]
#![feature(total_cmp)]
#![allow(clippy::large_enum_variant, dead_code)]
// FIXME: Remove allow once ready for first release
#![allow(unused_imports, unused_variables, incomplete_features)]
#![warn(rust_2018_idioms)]

mod dbus;
mod systemd;

#[cfg(test)]
mod tests {
    use tracing::{error, warn};

    use crate::systemd::{Mode, Systemd1Manager, SystemdManager};

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn can_start_unit() {
        let systemd = SystemdManager::default();
        // QUESTION: Include a systemd service for integration testing? Is unit testing enough?
        // This isn't unit testing; this is crate, d-bus, and systemd integration testing.
        let unit_start_result = systemd.start_unit("sshd.service", &Mode::Replace).await;
        assert_eq!(unit_start_result.is_ok(), true);
    }
}
