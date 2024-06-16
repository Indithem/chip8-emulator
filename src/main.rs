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
    let args = Args::parse();

    if !args.no_debug {
        if args.verbose_debug {
            tracing_subscriber::fmt().init();
        } else {
            tracing_subscriber::fmt()
                .with_env_filter("chip_8_emulator")
                .init();
        }
    }

    let initialize_all_components = Arc::new(Barrier::new(2));

    // todo: graphics memory could be shared in other types
    let graphics_mem = Arc::new(RwLock::new(graphics::GraphicsMemory::new()));

    let rom = std::fs::File::open(args.rom_path).expect("Unable to open the file");
    let graphics_mem_cpu_cpy = Arc::clone(&graphics_mem);
    let cpu_thread_blocker = Arc::clone(&initialize_all_components);
    let pauses = args.pauses;

    // todo: when cpu sneezes, the rest of the components should catch a cold
    thread::Builder::new()
        .name("CPU".to_string())
        .spawn(move || {
            let mut cpu = cpu::CPU::new(rom, graphics_mem_cpu_cpy);
            cpu_thread_blocker.wait();
            tracing::info!("CPU thread started");
            if pauses {
                cpu.run_with_pauses()
            } else {
                cpu.run()
            };
        })
        .unwrap();

    graphics::main_thread(graphics_mem, initialize_all_components);
}

#[derive(clap::Parser)]
struct Args {
    /// Path to the rom file
    rom_path: String,

    /// Donot print debug logs
    #[clap(long, default_value = "false")]
    no_debug: bool,

    /// Run with [p]auses
    ///
    /// the emulator will wait for input after each cycle
    #[clap(short, long, default_value = "false")]
    pauses: bool,

    /// Verbose debug logs
    ///
    /// includes logs from dependencies, such as wgpu,
    /// this may include logs for device specific messages
    #[clap(long, default_value = "false", alias = "vdbg")]
    verbose_debug: bool,
}

use std::{
    sync::{Arc, Barrier, RwLock},
    thread,
};

use clap::Parser;
