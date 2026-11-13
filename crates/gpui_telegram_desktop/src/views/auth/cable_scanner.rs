//! Upstream Source: Telegram/SourceFiles/webauthn/cable_scanner.h
//! Subsystem: webauthn | Feature ID: TCG-N01 | Disposition: rust_domain_or_server_1to1

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub trait BleScannerTrait: Send + Sync {
    fn start(&self) -> bool;
    fn stop(&self);
    fn is_available(&self) -> bool;
}

#[derive(Debug, Clone, Default)]
pub struct CableScanner {
    is_scanning: Arc<AtomicBool>,
}

impl CableScanner {
    pub fn new() -> Self {
        Self {
            is_scanning: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start(&self) -> bool {
        self.is_scanning.store(true, Ordering::SeqCst);
        true
    }

    pub fn stop(&self) {
        self.is_scanning.store(false, Ordering::SeqCst);
    }

    pub fn is_scanning(&self) -> bool {
        self.is_scanning.load(Ordering::SeqCst)
    }

    pub fn is_available(&self) -> bool {
        true
    }
}

impl BleScannerTrait for CableScanner {
    fn start(&self) -> bool {
        CableScanner::start(self)
    }

    fn stop(&self) {
        CableScanner::stop(self);
    }

    fn is_available(&self) -> bool {
        CableScanner::is_available(self)
    }
}
