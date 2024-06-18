/// This will get shared between the threads.
pub struct BaseTimer {
    count: u8,
}
impl BaseTimer {
    pub fn new() -> Self {
        BaseTimer {count: 0}
    }

    pub fn set_timer(&mut self, time: u8) {
        self.count = time;
    }

    pub fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1)
    }

    pub fn read(&self) -> u8 {
        self.count
    }
}