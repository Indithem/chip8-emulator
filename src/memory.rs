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

impl Memory {
    /// The memory address where the instructions would start
    pub const INSTRUCTIONS_START_ADDRESS: usize = 0x200;
    pub const DIGITS_FONTS_START_ADDRESS: usize = 0x000;
    const DIGITS_FONTS: [u8; 80] = [
        0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
        0x20, 0x60, 0x20, 0x20, 0x70, // 1
        0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
        0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
        0x90, 0x90, 0xF0, 0x10, 0x10, // 4
        0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
        0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
        0xF0, 0x10, 0x20, 0x40, 0x40, // 7
        0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
        0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
        0xF0, 0x90, 0xF0, 0x90, 0x90, // A
        0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
        0xF0, 0x80, 0x80, 0x80, 0xF0, // C
        0xE0, 0x90, 0x90, 0x90, 0xE0, // D
        0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
        0xF0, 0x80, 0xF0, 0x80, 0x80, // F
    ];

    pub fn load_instructions(mut file: std::fs::File) -> Result<Memory, &'static str> {
        let mut data = [0; 4096];
        data[Self::DIGITS_FONTS_START_ADDRESS
            ..Self::DIGITS_FONTS_START_ADDRESS + Self::DIGITS_FONTS.len()]
            .copy_from_slice(&Self::DIGITS_FONTS);
        tracing::info!("Loading instructions into memory");
        let file_size = file.metadata().unwrap().len() as usize;
        if file_size > 4096 - Self::INSTRUCTIONS_START_ADDRESS {
            return Err("Instructions are too large to fit in memory");
        }
        file.read(&mut data[Self::INSTRUCTIONS_START_ADDRESS..])
            .expect("Unable to read the file");
        Ok(Memory(data))
    }

    /// Gets the address of digit's font in memory
    pub const fn get_digit_address(digit: u8) -> usize {
        Self::DIGITS_FONTS_START_ADDRESS + (digit as usize * 5)
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
impl IndexMut<std::ops::Range<usize>> for Memory {
    fn index_mut(&mut self, index: std::ops::Range<usize>) -> &mut Self::Output {
        &mut self.0[index]
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
