//! Structure of the project:
//! - any data that is meant to be used is made as a struct.
//! The struct will be initialized in the main thread.
//! (I actually wanted to initialized and locate it on the static/.data section,
//!  but rust doesnt seem good with mutable statics)
//! All those data, if needed, will be wrapped in some kind of std::sync lock.
//! also, they shall be on the heap.

mod cpu;
mod graphics;
mod input;
mod memory;
mod sound;
mod timers;

fn main() {
    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::init();

    // todo: graphics memory could be shared in other types
    let graphics_mem = Arc::new(RwLock::new(memory::GraphicsMemory::new()));

    let rom = std::fs::File::open("test roms/1-chip8-logo.ch8").expect("Unable to open the file");
    let graphics_mem_cpu_cpy = Arc::clone(&graphics_mem);

    thread::Builder::new()
        .name("CPU".to_string())
        .spawn(move || {
            cpu::cpu_thread(graphics_mem_cpu_cpy, rom);
        })
        .unwrap();

    graphics::main_thread(graphics_mem);
}

#[allow(unused_imports)]
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};
