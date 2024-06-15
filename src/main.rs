//! Structure of the project:
//! - any data that is meant to be used is made as a struct.
//! The struct will be initialized in the main thread.
//! (I actually wanted to initialized and locate it on the static/.data section,
//!  but rust doesnt seem good with mutable statics)
//! All those data, if needed, will be wrapped in some kind of std::sync lock.
//! also, they shall be on the heap.

#[allow(unused)]
mod cpu;
mod graphics;
mod input;
#[allow(unused)]
mod memory;
mod sound;
mod timers;

fn main() {

    #[cfg(debug_assertions)]
    tracing_subscriber::fmt::init();

    // todo: graphics memory could be shared in other types
    let graphics_mem = Arc::new(RwLock::new(memory::GraphicsMemory::new()));

    
    graphics::main_thread(graphics_mem);
}

#[allow(unused_imports)]
use std::{
    sync::{Arc, Mutex, RwLock},
    thread,
};