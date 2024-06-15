/// The memory of the virtual machine.
///
/// total = 4KiB,
/// 2**12 possible addresses
///
/// ## Note
/// Donot allocate it on stack, as it itself is 4KiB
struct Memory([u8; 4096]);

/// Memory for the registers
struct RegisterMemory([u8; 16]);
#[rustfmt::skip]
/// All registers that are available for use.
enum Registers{
    V0, V1, V2, V3, V4, V5, V6, V7, V8, V9, VA, VB, VC, VD, VE, 
    /// special register for carry flag
    VF
}

/// Stores the return addresses
struct Stack(Vec<u16>);


/// The complete memory assosciated to graphics
pub struct GraphicsMemory([bool; TOTAL_PIXELS]);

const TOTAL_PIXELS: usize =
    (crate::graphics::SCREEN_SIZE.0 * crate::graphics::SCREEN_SIZE.1) as usize;

impl GraphicsMemory {
    pub fn new() -> Self {
        tracing::info!("Initializing graphics memory");
        Self([false; TOTAL_PIXELS])
    }

    /// Make a iterator over the pixels as registered in the graphics memory
    pub fn iter<'it> (&self) -> MemoryIterator<bool> {
        MemoryIterator {
            index: 0,
            data_slice: &self.0,
            max_index: TOTAL_PIXELS,
        }
    }
}

/// A generic iterator for the memory structs
pub struct MemoryIterator<'it, T> {
    index: usize,
    data_slice: &'it [T],
    max_index: usize,
}
impl<'it,T> Iterator for MemoryIterator<'it, T> {
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