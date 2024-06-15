pub fn cpu_thread(graphics_mem: Arc<RwLock<GraphicsMemory>>, rom: std::fs::File) {
    let mut cpu = CPU::new(rom, graphics_mem);
    tracing::info!("CPU thread started");
    cpu.run();
}

struct CPU {
    /// Stores the return addresses
    stack: Vec<u16>,
    i_register: memory::IRegister,
    register_memory: [u8; 16],
    memory: memory::Memory,
    graphics_memory: Arc<RwLock<GraphicsMemory>>,
    current_opcode: usize,
}

impl CPU {
    fn new(file: std::fs::File, graphics_memory: Arc<RwLock<GraphicsMemory>>) -> Self {
        CPU {
            stack: Vec::new(),
            i_register: memory::IRegister::new(),
            register_memory: [0; 16],
            memory: memory::Memory::load_instructions(file),
            graphics_memory,
            current_opcode: memory::Memory::INSTRUCTIONS_START_ADDRESS,
        }
    }

    fn run(&mut self) -> ! {
        loop {
            let opcode = self.fetch_opcode();
            self.run_opcode(opcode);
        }
    }

    fn fetch_opcode(&mut self) -> u16 {
        let opcode = (self.memory[self.current_opcode] as u16) << 8
            | self.memory[self.current_opcode + 1] as u16;
        self.current_opcode += 2;
        opcode
    }

    fn run_opcode(&mut self, opcode: u16) {
        match opcode {
            _ => panic!("Unknown state, {}", self.dump(opcode)),
        }
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
            self.stack, self.i_register, self.register_memory, self.current_opcode, opcode.into().map(|x| format!("{:04X}", x))
        )
    }
}

/// All registers that are available for use.
#[rustfmt::skip]
enum Registers{
    V0=0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, 
    /// special register for carry flag
    VF
}

use std::sync::{Arc, RwLock};

use crate::memory::{self, GraphicsMemory};
