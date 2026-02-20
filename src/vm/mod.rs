pub mod registers;
pub mod isa;
pub mod assembler;

use std::path::Path;
use registers::*;

#[derive(Clone)]
pub struct M0_32 {
    pub registers: [u32; 16],
    pub mem: Box<[u8; 4_294_967_296]>,
    pub is_running: bool,
}

impl M0_32 {
    pub fn new() -> Self {
        let mut registers = [0u32; 16];
        registers[PC as usize] = 3_221_225_472u32;
        registers[SP as usize] = 3_221_225_471u32;

        Self {
            registers,
            mem: vec![0u8; 4_294_967_296]
                .into_boxed_slice()
                .try_into()
                .unwrap(),
            is_running: false,
        }
    }

    pub fn load_program(&mut self, program: &[u8]) {
        assert!(program.len() <= 1_073_741_824, "program length exceeds limit of 1GB");

        self.mem[3_221_225_472..3_221_225_472 + program.len()].copy_from_slice(program);
    }

    pub fn load_program_from_path<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        use std::fs;

        let bytes = fs::read(path)
            .map_err(|err| err.to_string())?;

        self.load_program(&bytes);
        Ok(())
    }

    pub fn run(&mut self) {
        self.is_running = true;

        while self.is_running {
            self.cycle();
        }
    }

    pub fn cycle(&mut self) {
        let pc = self.registers[PC as usize];
        let inst = u64::from_le_bytes([
            self.mem[pc as usize],
            self.mem[pc.wrapping_add(1) as usize],
            self.mem[pc.wrapping_add(2) as usize],
            self.mem[pc.wrapping_add(3) as usize],
            self.mem[pc.wrapping_add(4) as usize],
            self.mem[pc.wrapping_add(5) as usize],
            0u8,
            0u8
        ].try_into().unwrap());
        self.registers[PC as usize] = pc.wrapping_add(6);

        let opcode = (inst & 0xFF) as u8;
        match opcode {
            isa::NOP => (),
            isa::HLT => self.is_running = false,
            isa::LDI => {
                let dest = ((inst >> 8) & 0xF) as usize;

                let imm = ((inst >> 12) & 0xFFFFFFFF) as u32;
                self.registers[dest] = imm;
            },
            isa::ADD => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = self.registers[src1].wrapping_add(self.registers[src2]);
            },
            isa::SUB => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = self.registers[src1].wrapping_sub(self.registers[src2]);
            },
            isa::BOR => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = self.registers[src1] | self.registers[src2];
            },
            isa::BAND => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = self.registers[src1] & self.registers[src2];
            },
            isa::BXOR => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = self.registers[src1] ^ self.registers[src2];
            },
            isa::BNOT => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;

                self.registers[dest] = !self.registers[src];
            },
            isa::LOR => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] != 0 || self.registers[src2] != 0) as u32;
            },
            isa::LAND => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] != 0 && self.registers[src2] != 0) as u32;
            },
            isa::LXOR => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = ((self.registers[src1] != 0) != (self.registers[src2] != 0)) as u32;
            },
            isa::LNOT => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;

                self.registers[dest] = (self.registers[src] == 0) as u32;
            },
            isa::SB => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;

                self.mem[self.registers[dest] as usize]
                    = (self.registers[src] & 0xFF) as u8;
            },
            isa::SW => {
                let dest = self.registers[((inst >> 8) & 0xF) as usize];
                let src = ((inst >> 12) & 0xF) as usize;
                let bytes = self.registers[src].to_le_bytes();
                
                for i in 0..4 {
                    self.mem[dest.wrapping_add(i) as usize] = bytes[i as usize];
                }
            },
            isa::LBS => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;

                self.registers[dest] = self.mem[src] as i8 as i32 as u32;
            },
            isa::LBU => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;

                self.registers[dest] = self.mem[src] as u32;
            },
            isa::LW => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = (inst >> 12) & 0xF;

                let bytes = [
                    self.mem[src as usize],
                    self.mem[src.wrapping_add(1) as usize],
                    self.mem[src.wrapping_add(2) as usize],
                    self.mem[src.wrapping_add(3) as usize],
                ];

                self.registers[dest] = u32::from_le_bytes(bytes);
            },
            isa::JMP => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let is_relative = ((inst >> 12) & 0x1) as u8;

                if is_relative != 0 {
                    let jmp = self.registers[dest] as i32;
                    if jmp.is_negative() {
                        self.registers[PC as usize] = pc.saturating_sub(jmp.abs() as u32);
                    } else {
                        self.registers[PC as usize] = pc.saturating_add(jmp as u32);
                    }
                } else {
                    self.registers[PC as usize] = self.registers[dest];
                }
            },
            isa::BRIF => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;
                let is_relative = ((inst >> 16) & 0x1) as u8;

                if self.registers[src] != 0 {
                    if is_relative != 0 {
                        let jmp = self.registers[dest] as i32;
                        if jmp.is_negative() {
                            self.registers[PC as usize] = pc.saturating_sub(jmp.abs() as u32);
                        } else {
                            self.registers[PC as usize] = pc.saturating_add(jmp as u32);
                        }
                    } else {
                        self.registers[PC as usize] = self.registers[dest];
                    }
                }
            },
            isa::CAL => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let is_relative = ((inst >> 12) & 0x1) as u8;

                let ret_addr = pc.wrapping_add(6).to_le_bytes();
                let sp = self.registers[SP as usize];
                for i in 0..4 {
                    self.mem[sp.wrapping_sub(i) as usize] = ret_addr[3 - i as usize];
                }
                self.registers[SP as usize] = sp.wrapping_sub(4);
                if is_relative != 0 {
                    let jmp = self.registers[dest] as i32;
                    if jmp.is_negative() {
                        self.registers[PC as usize] = pc.saturating_sub(jmp.abs() as u32);
                    } else {
                        self.registers[PC as usize] = pc.saturating_add(jmp as u32);
                    }
                } else {
                    self.registers[PC as usize] = self.registers[dest];
                }
            },
            isa::CAIF => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src = ((inst >> 12) & 0xF) as usize;
                let is_relative = ((inst >> 16) & 0x1) as u8;

                if self.registers[src] != 0 {
                    let ret_addr = pc.wrapping_add(6).to_le_bytes();
                    let sp = self.registers[SP as usize];
                    for i in 0..4 {
                        self.mem[sp.wrapping_sub(i) as usize] = ret_addr[3 - i as usize];
                    }
                    self.registers[SP as usize] = sp.wrapping_sub(4);
                    if is_relative != 0 {
                        let jmp = self.registers[dest] as i32;
                        if jmp.is_negative() {
                            self.registers[PC as usize] = pc.saturating_sub(jmp.abs() as u32);
                        } else {
                            self.registers[PC as usize] = pc.saturating_add(jmp as u32);
                        }
                    } else {
                        self.registers[PC as usize] = self.registers[dest];
                    }
                }
            },
            isa::RET => {
                let mut bytes = [0; 4];
                for i in 0..4 {
                    let sp = self.registers[SP as usize];
                    bytes[i] = self.mem[sp as usize];
                    self.registers[SP as usize] = self.registers[SP as usize].wrapping_add(1);
                }

                self.registers[PC as usize] = u32::from_le_bytes(bytes);
            },
            isa::EQ => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] == self.registers[src2]) as u32;
            },
            isa::NE => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] != self.registers[src2]) as u32;
            },
            isa::GT => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] > self.registers[src2]) as u32;
            },
            isa::LT => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] < self.registers[src2]) as u32;
            },
            isa::GE => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] >= self.registers[src2]) as u32;
            },
            isa::LE => {
                let dest = ((inst >> 8) & 0xF) as usize;
                let src1 = ((inst >> 12) & 0xF) as usize;
                let src2 = ((inst >> 16) & 0xF) as usize;

                self.registers[dest] = (self.registers[src1] <= self.registers[src2]) as u32;
            },
            _ => (),
        }

        self.registers[0] = 0u32;
    }
}