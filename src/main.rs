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

    let sync_barrier = Arc::new(Barrier::new(4));

    let delay_timer = Arc::new(RwLock::new(timers::BaseTimer::new()));

    // todo: graphics memory could be shared in other types
    let graphics_mem = Arc::new(RwLock::new(graphics::GraphicsMemory::new()));

    let rom = std::fs::File::open(args.rom_path).expect("Unable to open the file");
    let graphics_mem_cpu_cpy = Arc::clone(&graphics_mem);
    let cpu_thread_blocker = Arc::clone(&sync_barrier);
    let pauses = args.pauses;
    let cpu_delay_timer = Arc::clone(&delay_timer);
    let (tx, rx) = std::sync::mpsc::channel();
    let (sound_tx, sound_rx) = std::sync::mpsc::channel();

    // cpu thread
    // todo: when cpu sneezes, the rest of the components should catch a cold
    // or maybe not?
    thread::Builder::new()
        .name("CPU".to_string())
        .spawn(move || {
            let cpu_constructed = cpu::CPU::new(
                rom,
                graphics_mem_cpu_cpy,
                cpu_delay_timer,
                input::InpuState::new(rx),
                sound_tx,
            );
            // no unblock before panicing the thread, else the  window becomes unresponsive
            cpu_thread_blocker.wait();
            tracing::info!("CPU thread started");
            let mut cpu = cpu_constructed.expect("Failed to construct the CPU");
            if pauses {
                cpu.run_with_pauses()
            } else {
                cpu.run()
            };
        })
        .unwrap();

    let delay_timer_sync = Arc::clone(&sync_barrier);
    // delay timer thread
    thread::Builder::new()
        .name("Delay Timer".to_string())
        .spawn(move || {
            delay_timer_sync.wait();
            tracing::info!("Delay Timer thread started");
            loop {
                std::thread::sleep(std::time::Duration::from_micros(1_000_000 / 60)); // 60Hz
                delay_timer.write().unwrap().decrement();
            }
        })
        .unwrap();

    let sound_sync = Arc::clone(&sync_barrier);
    // sound thread
    thread::Builder::new()
        .name("Sound".to_string())
        .spawn(move || {
            sound::main_thread(sound_rx, sound_sync);
        })
        .unwrap();

    graphics::main_thread(graphics_mem, sync_barrier, tx);
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
