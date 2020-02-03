mod registers;
mod instructions;

use registers::Registers;
use instructions::Instruction;
use instructions::Target;

pub struct CPU {
    registers: Registers,
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            registers: Registers::new(),
        }
    }

    fn execute(&mut self, instruction: Instruction ) {
        match instruction {
            Instruction::ADD(target) => {
                match target {
                    Target::C => {
                        let val = self.registers.c;
                        let new_val = self.add(val);
                        self.registers.a = new_val;
                    }
                    _ => {

                    }
                }
            }
            _ => {
                println!("Unknown instruction!");
            }
        }
    }

    fn add(&mut self, value: u8) -> u8{
        let (new_value, did_overflow) = self.registers.a.overflowing_add(value);
        self.registers.f.zero = new_value == 0;
        self.registers.f.subtract = false;
        self.registers.f.half_carry = (self.registers.a & 0xF) + (value & 0xF) > 0xF;
        self.registers.f.carry = did_overflow; 
        new_value
    }
}