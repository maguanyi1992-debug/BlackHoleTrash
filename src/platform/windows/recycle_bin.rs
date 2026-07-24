//! Recycle Bin integration is intentionally disabled in the visual-only build.

use super::{EventSender, PlatformEvent};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct RecycleResult {
    pub generation: u64,
    pub paths: Vec<PathBuf>,
    pub succeeded: bool,
    pub aborted: bool,
    pub message: Option<String>,
}

/// Compatibility stub retained for the unchanged event loop.
/// In the visual-only build no OLE drop target exists, so this should never be
/// called during normal operation. It never deletes or moves any file.
pub fn recycle_async(generation: u64, paths: Vec<PathBuf>, sender: EventSender) {
    let result = RecycleResult {
        generation,
        paths,
        succeeded: false,
        aborted: true,
        message: Some("Recycle Bin integration is disabled in this visual-only build".into()),
    };
    let _ = sender.try_send(PlatformEvent::RecycleFinished(result));
}

/// No error dialog is shown because file recycling is not a supported action.
pub fn show_recycle_failure(_result: &RecycleResult) {}
