pub mod readme;

use std::path::PathBuf;

/// Shared state during rendering of a single page.
pub struct RenderContext {
    pub page_dir: PathBuf,
    pub dry_run: bool,
    pub should_overwrite: bool,
}

impl RenderContext {
    pub fn new(page_dir: PathBuf, dry_run: bool, should_overwrite: bool) -> Self {
        Self {
            page_dir,
            dry_run,
            should_overwrite,
        }
    }

    /// Return the path to the `embedded` subdirectory for the current page.
    pub fn embedded_dir(&self) -> PathBuf {
        self.page_dir.join("embedded")
    }
}
