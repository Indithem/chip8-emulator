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

/// The complete memory assosciated to graphics
pub struct GraphicsMemory([bool; GraphicsMemory::TOTAL_PIXELS]);

impl GraphicsMemory {
    const TOTAL_PIXELS: usize =
        (crate::graphics::SCREEN_SIZE.0 * crate::graphics::SCREEN_SIZE.1) as usize;

    pub fn new() -> Self {
        tracing::info!("Initializing graphics memory");
        let mut data = [false; Self::TOTAL_PIXELS];
        // a simple design of a 5x5 square
        data[0] = true;
        data[1] = true;
        data[2] = true;
        data[3] = true;
        data[4] = true;
        data[5] = true;
        data[10] = true;
        data[15] = true;
        data[20] = true;
        data[21] = true;
        data[22] = true;
        data[23] = true;
        data[24] = true;

        data[63] = true;
        data[63+64] = true;
        data[63+128] = true;
        data[63+192] = true;
        data[63+256] = true;
        data[63+320] = true;
        data[63+384] = true;
        Self(data)
    }

    /// Make a iterator over the pixels as registered in the graphics memory
    pub fn iter<'it>(&self) -> MemoryIterator<bool> {
        MemoryIterator {
            index: 0,
            data_slice: &self.0,
            max_index: Self::TOTAL_PIXELS,
        }
    }

    pub fn clear_screen(&mut self) {
        self.0 = [false; Self::TOTAL_PIXELS];
    }

    #[rustfmt::skip]
    pub fn display_sprite(&mut self, x: u8, y: u8, sprite: &[u8]) -> bool {
        const MAX_X: usize = crate::graphics::SCREEN_SIZE.0 as usize;
        const MAX_Y: usize = crate::graphics::SCREEN_SIZE.1 as usize;
        let x = x as usize % MAX_X;
        let y = y as usize % MAX_Y;

        let mut collision = false;
        for (y_off, sprite_byte) in sprite.iter().enumerate() {
            let y = y + y_off;
            if y >= MAX_Y { Self::report_out_of_screen(x, y); continue; }
            for x_off in 0..8 {
                let x = x + x_off;
                if x >= MAX_X { Self::report_out_of_screen(x, y); continue; }
                let pixel = &mut self.0[y*MAX_X + x];
                let sprite_pixel = (sprite_byte >> (7 - x_off)) & 0x1 == 1;
                collision |= *pixel && sprite_pixel;
                *pixel ^= sprite_pixel;
            }
        }
        collision
    }

    fn report_out_of_screen(x: usize, y: usize) {
        tracing::warn!("Sprite out of screen, x: {}, y: {}, Clipping it!", x, y);
    }

    #[cfg(debug_assertions)]
    #[allow(unused)]
    /// For sake of testing, negate all the pixels
    pub fn negate(&mut self) {
        for pixel in self.0.iter_mut() {
            *pixel = !*pixel;
        }
    }
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
    index: usize,
    data_slice: &'it [T],
    max_index: usize,
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
