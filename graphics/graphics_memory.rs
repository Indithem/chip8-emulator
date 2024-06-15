/// The complete memory assosciated to graphics
pub struct GraphicsMemory([bool; GraphicsMemory::TOTAL_PIXELS]);

impl GraphicsMemory {
    const TOTAL_PIXELS: usize =
        (crate::SCREEN_SIZE.0 * crate::SCREEN_SIZE.1) as usize;

    pub fn new() -> Self {
        tracing::info!("Initializing graphics memory");
        // alternating pixels
        let mut data = [false; Self::TOTAL_PIXELS];
        for (i, pixel) in data.iter_mut().enumerate() {
            *pixel = i % 2 == 0;
        }
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

    #[cfg(debug_assertions)]
    #[allow(unused)]
    /// For sake of testint, negate all the pixels
    pub fn negate(&mut self) {
        for pixel in self.0.iter_mut() {
            *pixel = !*pixel;
        }
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

extern crate memory;
use self::memory::MemoryIterator;

use std::ops::{Index, IndexMut};
