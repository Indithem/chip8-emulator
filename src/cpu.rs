pub struct CPU {
    /// Stores the return addresses
    stack: Vec<usize>,
    i_register: u16,
    register_memory: [u8; 16],
    memory: memory::Memory,
    graphics_memory: Arc<RwLock<GraphicsMemory>>,
    instruction_ptr: usize,
}

impl CPU {
    pub fn new(file: std::fs::File, graphics_memory: Arc<RwLock<GraphicsMemory>>) -> Self {
        CPU {
            stack: Vec::new(),
            i_register: 0,
            register_memory: [0; 16],
            memory: memory::Memory::load_instructions(file),
            graphics_memory,
            instruction_ptr: memory::Memory::INSTRUCTIONS_START_ADDRESS,
        }
    }

    #[rustfmt::skip]
    pub fn run_with_pauses(&mut self) -> ! {
        let mut cycles = 0u64;
        loop {
            let opcode = self.fetch_opcode();
            pause(format!("Starting Cycle: {}, CPU state: {}", cycles, self.dump_without_memory(opcode)));
            self.follow_isa(opcode);
            cycles += 1;
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let opcode = self.fetch_opcode();
            self.follow_isa(opcode);
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode = (self.memory[self.instruction_ptr] as u16) << 8
            | self.memory[self.instruction_ptr + 1] as u16;
        self.instruction_ptr += 2;
        opcode
    }

    fn dump(&self, opcode: impl Into<Option<u16>>) -> String {
        format!(
            "CPU Dump:
            Memory: {:?}
            Stack: {:?}
            {:?}
            Registry Memory: {:?}
            Current Opcode pointer: 0x{:04X}
            Opcode: {:?}
            ",
            self.memory,
            self.stack,
            self.i_register,
            self.register_memory,
            self.instruction_ptr,
            opcode.into().map(|x| format!("{:04X}", x))
        )
    }

    fn dump_without_memory(&self, opcode: impl Into<Option<u16>>) -> String {
        format!(
            "CPU Dump:
            Stack: {:?}
            {:?}
            Registry Memory: {:?}
            Current Opcode pointer: 0x{:04X}
            Opcode: {:?}
            ",
            self.stack,
            self.i_register,
            self.register_memory,
            self.instruction_ptr,
            opcode.into().map(|x| format!("{:04X}", x))
        )
    }
}

fn pause(msg: String) {
    tracing::info!("{}", msg);
    stdin().read(&mut [0, 0]).unwrap();
}

use std::io::{stdin, Read};
use std::sync::{Arc, RwLock};

use crate::memory;
use crate::graphics::GraphicsMemory;

/// Has function for decoding and executing the opcodes
mod isa;
