/// The memory of the virtual machine.
///
/// total = 4KiB,
/// 2**12 possible addresses
/// addressable from 0x000 to 0xFFF
///
/// ## Note
/// Donot allocate it on stack, as it itself is 4KiB
///     nah, I did.
#[derive(Debug)]
pub struct Memory([u8; 4096]);

/// The I register
///
/// # Safety
///  The I register is ideally 12 bits wide, use with caution
#[derive(Debug)]
pub struct IRegister {
    data: u16,
    /// Is the I-register modified before?
    ///
    /// maybe this is not needed.
    ///
    /// This is needed because, in CHIP-8,
    /// `
    ///  No instructions exist to modify the I register after it is set to a given value.
    /// `
    assigned: bool,
}

impl Memory {
    /// The memory address where the instructions would start
    pub const INSTRUCTIONS_START_ADDRESS: usize = 0x200;

    pub fn load_instructions(mut file: std::fs::File) -> Self {
        let mut data = [0; 4096];
        tracing::info!("Loading instructions into memory");
        file.read(&mut data[Self::INSTRUCTIONS_START_ADDRESS..])
            .expect("Unable to read the file");
        Memory(data)
    }
}

impl IRegister {
    pub fn new() -> Self {
        Self {
            data: 0,
            assigned: false,
        }
    }

    pub fn store(&mut self, value: u16) {
        self.data = value;
        self.assigned = true;
    }

    pub fn get(&self) -> u16 {
        self.data
    }
}

/// A generic iterator for the memory structs
pub struct MemoryIterator<'it, T> {
    pub index: usize,
    pub data_slice: &'it [T],
    pub max_index: usize,
}
impl<'it, T> Iterator for MemoryIterator<'it, T> {
    type Item = &'it T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.max_index {
            let item = &self.data_slice[self.index];
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl Index<std::ops::Range<usize>> for Memory {
    type Output = [u8];

    fn index(&self, index: std::ops::Range<usize>) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl Index<usize> for GraphicsMemory {
    type Output = bool;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}
impl IndexMut<usize> for GraphicsMemory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

use std::{
    io::Read,
    ops::{Index, IndexMut},
};

use crate::graphics::GraphicsMemory;