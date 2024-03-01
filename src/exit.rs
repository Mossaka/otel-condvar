use std::sync::{Condvar, Mutex};

#[derive(Default)]
pub struct ExitSignal(Mutex<bool>, Condvar);

#[allow(clippy::mutex_atomic)]
impl ExitSignal {
    /// Set exit signal to shutdown shim server.
    pub fn signal(&self) {
        let (lock, cvar) = (&self.0, &self.1);
        let mut exit = lock.lock().unwrap();
        *exit = true;
        cvar.notify_all();
    }

    /// Wait for the exit signal to be set.
    pub fn wait(&self) {
        let (lock, cvar) = (&self.0, &self.1);
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }
    }
}
