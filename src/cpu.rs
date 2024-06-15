pub struct CPU {
    /// Stores the return addresses
    stack: Vec<usize>,
    i_register: memory::IRegister,
    register_memory: [u8; 16],
    memory: memory::Memory,
    graphics_memory: Arc<RwLock<GraphicsMemory>>,
    instruction_ptr: usize,
}

impl CPU {
    pub fn new(file: std::fs::File, graphics_memory: Arc<RwLock<GraphicsMemory>>) -> Self {
        CPU {
            stack: Vec::new(),
            i_register: memory::IRegister::new(),
            register_memory: [0; 16],
            memory: memory::Memory::load_instructions(file),
            graphics_memory,
            instruction_ptr: memory::Memory::INSTRUCTIONS_START_ADDRESS,
        }
    }

    pub fn run(&mut self) -> ! {
        loop {
            let opcode = self.fetch_opcode();
            self.run_opcode(opcode);
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode = (self.memory[self.instruction_ptr] as u16) << 8
            | self.memory[self.instruction_ptr + 1] as u16;
        self.instruction_ptr += 2;
        opcode
    }

    #[rustfmt::skip]
    fn dump(&self, opcode: impl Into<Option<u16>>) -> String {
        format!(
            "CPU Dump:
            Stack: {:?}
            {:?}
            Registry Memory: {:?}
            Current Opcode pointer: 0x{:04X}
            Opcode: {:?}
            ",
            self.stack, self.i_register, self.register_memory, self.instruction_ptr, opcode.into().map(|x| format!("{:04X}", x))
        )
    }
}

use std::sync::{Arc, RwLock};

use crate::memory::{self, GraphicsMemory};

/// Has function for decoding and executing the opcodes
mod isa;