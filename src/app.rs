use std::sync::atomic::{AtomicBool, Ordering};

pub struct App {
    /// Whether or not the app received a sigint
    interrupted: AtomicBool,
    is_debug_flag: bool,
}

impl App {
    pub fn new(is_debug_flag: bool) -> Self {
        Self {
            interrupted: AtomicBool::new(false),
            is_debug_flag: is_debug_flag,
        }
    }

    pub fn is_debug_flag(&self) -> bool {
        self.is_debug_flag
    }

    pub fn is_interrupted(&self) -> bool {
        self.interrupted.load(Ordering::SeqCst)
    }

    pub fn interrupt(&self) {
        self.interrupted.store(true, Ordering::SeqCst);
    }
}
