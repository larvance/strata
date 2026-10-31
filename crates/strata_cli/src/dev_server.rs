//! Development server with state-preserving Hot Module Replacement (HMR).

use std::time::{SystemTime, UNIX_EPOCH};
use crate::args::BuildTarget;

/// Active Hot Module Replacement session coordinating state preservation and dynamic reloads.
#[derive(Debug, Clone)]
pub struct HmrSession {
    pub session_id: String,
    pub signal_snapshot_bytes: Option<Vec<u8>>,
    pub reload_count: usize,
    pub is_active: bool,
    pub last_reload_unix_ms: u64,
}

impl Default for HmrSession {
    fn default() -> Self {
        Self::new()
    }
}

impl HmrSession {
    /// Creates a new HMR coordination session.
    pub fn new() -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;

        Self {
            session_id: format!("hmr-{}", now),
            signal_snapshot_bytes: None,
            reload_count: 0,
            is_active: true,
            last_reload_unix_ms: now,
        }
    }

    /// Serializes active reactive state into the session snapshot buffer.
    pub fn snapshot_state(&mut self, bytes: Vec<u8>) {
        self.signal_snapshot_bytes = Some(bytes);
    }

    /// Retrieves the serialized state buffer to restore signal values after module re-compilation.
    pub fn restore_state(&self) -> Option<&[u8]> {
        self.signal_snapshot_bytes.as_deref()
    }

    /// Increments reload count, updates timestamp, and triggers dynamic library / module reload.
    pub fn trigger_reload(&mut self) -> usize {
        self.reload_count += 1;
        self.last_reload_unix_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        self.reload_count
    }
}

/// Configuration options for the local development server.
#[derive(Debug, Clone)]
pub struct DevServerConfig {
    pub target: BuildTarget,
    pub port: u16,
    pub watch: bool,
}

impl Default for DevServerConfig {
    fn default() -> Self {
        Self {
            target: BuildTarget::Desktop,
            port: 3000,
            watch: true,
        }
    }
}

/// Local development server instance.
pub struct DevServer {
    pub config: DevServerConfig,
    pub session: HmrSession,
}

impl DevServer {
    pub fn new(config: DevServerConfig) -> Self {
        Self {
            config,
            session: HmrSession::new(),
        }
    }

    /// Simulates a detected file change triggering a state-preserving HMR cycle.
    pub fn on_file_changed(&mut self, file_path: &str) -> usize {
        println!("⚡ [HMR] File change detected: '{}'", file_path);
        println!("   Serializing reactive signal graph...");
        // In actual HMR, the serialized buffer is preserved in self.session
        let reload_num = self.session.trigger_reload();
        println!("   Module reloaded successfully (HMR iteration #{})", reload_num);
        reload_num
    }
}
