impl super::CPU {
    /// completely inspired from https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
    pub fn follow_isa(&mut self, opcode: u16) {
        let register_x = ((opcode & 0x0F00) / 0x100) as usize;
        let register_y = ((opcode & 0x00F0) / 0x10) as usize;
        match opcode {
            0x00E0 => self.graphics_memory.write().unwrap().clear_screen(),

            #[rustfmt::skip]
            0x00EE => self.instruction_ptr = self.stack.pop().expect(&format!("No function call to return to, cpu state: {}", self.dump(opcode))),

            0..=0x0FFF => {
                tracing::error!("I dont understand this opcode, {}", opcode);
                self.stack.push(self.instruction_ptr);
                self.instruction_ptr = opcode as usize;
            }

            0x1000..=0x1FFF => self.instruction_ptr = (opcode - 0x1000) as usize,

            0x2000..=0x2FFF => {
                self.stack.push(self.instruction_ptr);
                self.instruction_ptr = (opcode - 0x2000) as usize;
            }

            0x3000..=0x3FFF => {
                let value = self.register_memory[register_x];
                let comparing_value = opcode & 0x00FF;
                if value == comparing_value as u8 {
                    self.instruction_ptr += 2;
                }
            }

            0x4000..=0x4FFF => {
                let value = self.register_memory[register_x];
                let comparing_value = opcode & 0x00FF;
                if value != comparing_value as u8 {
                    self.instruction_ptr += 2;
                }
            }

            #[rustfmt::skip]
            0x5000..=0x5FFF => {
                if opcode & 0x000F == 0 {
                    let v1 = self.register_memory[register_x];
                    let v2 = self.register_memory[register_y];
                    if v1 == v2 { self.instruction_ptr += 2; }
                } else { unreachable!("Unknown opcode, cpu state: {}", self.dump(opcode)) }
            },

            0x6000..=0x6FFF => {
                let value = opcode & 0x00FF;
                self.register_memory[register_x] = value as u8;
            }

            0x7000..=0x7FFF => {
                let value = (opcode & 0x00FF) as u8;
                let reg_mem = &mut self.register_memory[register_x];
                *reg_mem = reg_mem.wrapping_add(value);
            }

            0x8000..=0x8FFF => match opcode & 0x000F {
                0x0 => self.register_memory[register_x] = self.register_memory[register_y],
                0x1 => self.register_memory[register_x] |= self.register_memory[register_y],
                0x2 => self.register_memory[register_x] &= self.register_memory[register_y],
                0x3 => self.register_memory[register_x] ^= self.register_memory[register_y],
                0x4 => {
                    let (result, overflow) = self.register_memory[register_x]
                        .overflowing_add(self.register_memory[register_y]);
                    self.register_memory[register_x] = result;
                    self.register_memory[0xF] = overflow as u8;
                }
                0x5 => {
                    let (result, overflow) = self.register_memory[register_x]
                        .overflowing_sub(self.register_memory[register_y]);
                    self.register_memory[register_x] = result;
                    self.register_memory[0xF] = !overflow as u8;
                }
                0x6 => {
                    self.register_memory[0xF] = self.register_memory[register_y] & 0x1;
                    self.register_memory[register_x] = self.register_memory[register_y] >> 1;
                }
                0x7 => {
                    let (result, overflow) = self.register_memory[register_y]
                        .overflowing_sub(self.register_memory[register_x]);
                    self.register_memory[register_x] = result;
                    self.register_memory[0xF] = !overflow as u8;
                }
                0xE => {
                    self.register_memory[0xF] = self.register_memory[register_y] & 0x80;
                    self.register_memory[register_x] = self.register_memory[register_y] << 1;
                }

                _ => unreachable!("Unknown opcode, cpu state: {}", self.dump(opcode)),
            },

            #[rustfmt::skip]
            0x9000..=0x9FFF => {
                if opcode & 0x000F == 0 {
                    let v1 = self.register_memory[register_x];
                    let v2 = self.register_memory[register_y];
                    if v1 != v2 { self.instruction_ptr += 2; }
                } else { unreachable!("Unknown opcode, cpu state: {}", self.dump(opcode)) }
            },

            0xA000..=0xAFFF => self.i_register = opcode & 0x0FFF,

            #[rustfmt::skip]
            0xB000..=0xBFFF => {
                self.instruction_ptr = ((opcode & 0x0FFF) + self.register_memory[0] as u16) as usize % 0x10000;
            }

            0xC000..=0xCFFF => {
                let random_number = rand::random::<u8>();
                self.register_memory[register_x] = random_number & (opcode & 0x00FF) as u8;
            }

            0xD000..=0xDFFF => {
                self.register_memory[0xF] = self.graphics_memory.write().unwrap().display_sprite(
                    self.register_memory[register_x],
                    self.register_memory[register_y],
                    {
                        let n = (opcode & 0x000F) as usize;
                        let start_idx = self.i_register as usize;
                        &self.memory[start_idx..start_idx + n]
                    },
                ) as u8;
            }

            0xE00..=0xEFFF => {
                let function = (opcode & 0x00FF) as u8;
                match function {
                    0x9E => {
                        self.inputs.receive_keys();
                        if let Ok(key) = self.register_memory[register_x].try_into() {
                            if self.inputs.is_pressed(key) {
                                self.instruction_ptr += 2;
                            }
                        } else {
                            // error already logged in the try_into implementation
                        }
                    }
                    0xA1 => {
                        self.inputs.receive_keys();
                        if let Ok(key) = self.register_memory[register_x].try_into() {
                            if !self.inputs.is_pressed(key) {
                                self.instruction_ptr += 2;
                            }
                        } else {
                            // error already logged in the try_into implementation
                        }
                    }
                    _ => unreachable!("Unknown opcode, cpu state: {}", self.dump(opcode)),
                }
            }

            #[rustfmt::skip]
            0xF00..=0xFFFF => {
                let function = (opcode & 0x00FF) as u8;
                match function {
                    0x07 => self.register_memory[register_x] = self.delay_timer.read().unwrap().read(),
                    0x0A => self.register_memory[register_x] = self.inputs.wait_for_key().into(),
                    0x15 => self.delay_timer.write().unwrap().set_timer(self.register_memory[register_x]),
                    0x18 => self.sound_timer.send(self.register_memory[register_x]).unwrap(),

                    0x1E => self.i_register = self.i_register.wrapping_add(self.register_memory[register_x] as u16),

                    0x29 => {
                        let digit = self.register_memory[register_x];
                        let digit = if digit > 0xF {
                            tracing::warn!("Trying to get a digit font for a non digit value: {}", digit);
                            digit % 0xF
                        } else {digit};
                        self.i_register = memory::Memory::get_digit_address(digit) as u16;
                    },

                    0x33 => {
                        let addr = self.i_register as usize;
                        let slice = &mut self.memory[addr..addr + 3];
                        let val = self.register_memory[register_x];
                        let (hundreds, tens, ones) = (val / 100, (val % 100) / 10, val % 10);
                        slice.copy_from_slice(&[hundreds, tens, ones]);
                    }

                    0x55 => {
                        let addr = self.i_register as usize;
                        let slice = &mut self.memory[addr..addr + register_x + 1];
                        slice.copy_from_slice(&self.register_memory[..=register_x]);
                        self.i_register += register_x as u16 + 1;
                    }

                    0x65 => {
                        let addr = self.i_register as usize;
                        let slice = &self.memory[addr..addr + register_x + 1];
                        self.register_memory[..=register_x].copy_from_slice(slice);
                        self.i_register += register_x as u16 + 1;
                    }

                    _ => unreachable!("Unknown opcode, cpu state: {}", self.dump(opcode)),
                }
            }
        }
    }
}

use crate::memory;
