const BEEP_TIME: Duration = SIXTY_HZ.saturating_sub(Duration::from_micros(50));
const SIXTY_HZ: Duration = Duration::from_millis(1_000 / 60);

pub fn main_thread(rx: Receiver<u8>, barrier: Arc<Barrier>) {
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    
    let mut timer = timers::BaseTimer::new();
    
    barrier.wait();
    tracing::info!("Sound thread started");
    loop {
        std::thread::sleep(SIXTY_HZ);
        if let Ok(time) = rx.try_recv() {
            timer.set_timer(time);
        }
        timer.decrement();
        if timer.read() > 1 {
            // there is an audible click between the beeps.
            // i tried ~increase~ decrease the beep time inbetween the loops.
            sink.append(make_beep());
        }
    }
    
}

fn make_beep() -> TakeDuration<SineWave> {
    SineWave::new(440.).take_duration(BEEP_TIME)
}

use rodio::{
    source::{SineWave, Source, TakeDuration},
    OutputStream, Sink,
};
use std::{sync::{mpsc::Receiver, Arc, Barrier}, time::Duration};

use crate::timers;
