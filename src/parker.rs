use std::{sync, task, time};
/// [parking::Parker]
pub struct Parker {
    parked: sync::Mutex<bool>,
    cvar: sync::Condvar,
}
impl Parker {
    /// [parking::Parker::new]
    pub fn new() -> Parker {
        Parker {
            parked: sync::Mutex::new(false),
            cvar: sync::Condvar::new(),
        }
    }
    /// [parking::Parker::park]
    pub fn park(&self) {
        let mut parked = self.parked.lock().unwrap();
        *parked = true;
        let _parked = self.cvar.wait_while(parked, |parked| *parked).unwrap();
    }
    /// [parking::Parker::park_timeout]
    pub fn park_timeout(&self, duration: time::Duration) -> bool {
        let mut parked = self.parked.lock().unwrap();
        *parked = true;
        self.cvar
            .wait_timeout_while(parked, duration, |parked| *parked)
            .unwrap()
            .1
            .timed_out()
    }
    /// [parking::Parker::unpark]
    pub fn unpark(&self) {
        let mut parked = self.parked.lock().unwrap();
        *parked = false;
        self.cvar.notify_one();
    }
}
impl task::Wake for Parker {
    fn wake(self: sync::Arc<Self>) {
        self.unpark();
    }
    fn wake_by_ref(self: &sync::Arc<Self>) {
        self.unpark();
    }
}
