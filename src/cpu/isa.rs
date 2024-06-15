impl super::CPU {
    /// completely inspired from https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Instruction-Set
    pub fn run_opcode(&mut self, opcode: u16) {
        let register_x = (opcode & 0x0F00 >> 8) as usize;
        let register_y = (opcode & 0x00F0 >> 4) as usize;
        match opcode {
            0x00E0 => self.graphics_memory.write().unwrap().clear(),

            #[rustfmt::skip]
            0x00EE => self.instruction_ptr = self.stack.pop().expect(&format!("No function call to return to, cpu state: {}", self.dump(opcode))),

            0..=0x0FFF => {
                todo!("I dont understand this opcode, {}", opcode);
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

            0x5000..=0x5FFF => {
                let v1 = self.register_memory[register_x];
                let v2 = self.register_memory[register_y];
                if v1 == v2 {
                    self.instruction_ptr += 2;
                }
            }

            0x6000..=0x6FFF => {
                let value = opcode & 0x00FF;
                self.register_memory[register_x] = value as u8;
            }

            0x7000..=0x7FFF => {
                let value = opcode & 0x00FF;
                self.register_memory[register_x] += value as u8;
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
                    self.register_memory[0xF] = self.register_memory[register_x] & 0x1;
                    self.register_memory[register_x] = self.register_memory[register_y] >> 1;
                }
                0x7 => {
                    let (result, overflow) = self.register_memory[register_y]
                        .overflowing_sub(self.register_memory[register_x]);
                    self.register_memory[register_x] = result;
                    self.register_memory[0xF] = !overflow as u8;
                }
                0xE => {
                    self.register_memory[0xF] = self.register_memory[register_x] & 0x80;
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

            0xA000..=0xAFFF => self.i_register.store(opcode & 0x0FFF),

            0xB000..=0xBFFF => {
                self.instruction_ptr = ((opcode & 0x0FFF) + self.register_memory[0] as u16) as usize
            }

            0xC000..=0xCFFF => {
                let random_number = rand::random::<u8>();
                self.register_memory[register_x] = random_number & (opcode & 0x00FF) as u8;
            }

            0xD000..=0xDFFF => todo!(
                "Need to implement sprites yet!\nCPU State: {}",
                self.dump(opcode)
            ),

            _ => panic!("Unknown state, {}", self.dump(opcode)),
        }
    }
}
