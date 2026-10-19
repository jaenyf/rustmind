use std::sync::atomic::{AtomicBool, Ordering};

pub struct App {
    /// Whether or not the app received a sigint
    interrupted: AtomicBool,
}

impl App {
    pub fn new() -> Self {
        Self {
            interrupted: AtomicBool::new(false),
        }
    }

    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }
}
