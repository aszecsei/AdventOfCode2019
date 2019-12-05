use num_enum::TryFromPrimitive;
use std::convert::TryInto;

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
pub enum Opcodes {
    Addition = 1,
    Multiplication = 2,
    Input = 3,
    Output = 4,
    JumpIfTrue = 5,
    JumpIfFalse = 6,
    LessThan = 7,
    Equals = 8,
    Halt = 99,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
pub enum ParameterModes {
    Position = 0,
    Immediate = 1,
}

fn get_num(digits: &[u8]) -> u8 {
    let mut acc = 0u8;
    for d in digits {
        acc *= 10;
        acc += *d as u8;
    }
    acc
}

pub struct Program {
    pub data: Vec<i64>,
    pub pc: usize,
    pub inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    input_idx: usize,
}

impl Program {
    pub fn new(data: &[i64], inputs: &[i64]) -> Self {
        Program {
            data: data.to_vec(),
            pc: 0,
            inputs: inputs.to_vec(),
            outputs: vec![],
            input_idx: 0,
        }
    }

    fn get_val(&self, idx: i64, mode: ParameterModes) -> i64 {
        match mode {
            ParameterModes::Immediate => idx,
            ParameterModes::Position => self.data[idx as usize],
        }
    }

    pub fn step(&mut self) -> bool {
        let mut instruction = self.data[self.pc];
        let digits = {
            let mut digits = [0; 5];
            let mut index = 4;
            while instruction > 0 {
                digits[index] = (instruction % 10) as u8;
                instruction /= 10;
                index -= 1;
            }
            digits
        };

        // ABCDE
        // DE = two-digit opcode
        // C  = mode of 1st parameter
        // B  = mode of 2nd parameter
        // A  = mode of 3rd parameter

        let opcode: Opcodes = get_num(&digits[3..5]).try_into().unwrap();
        let param1_mode: ParameterModes = digits[2].try_into().unwrap();
        let param2_mode: ParameterModes = digits[1].try_into().unwrap();
        let _param3_mode: ParameterModes = digits[0].try_into().unwrap();

        match opcode {
            Opcodes::Addition => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                let out_addr = self.data[self.pc + 3] as usize;
                self.data[out_addr] = in1 + in2;
                self.pc += 4;
            }
            Opcodes::Multiplication => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                let out_addr = self.data[self.pc + 3] as usize;
                self.data[out_addr] = in1 * in2;
                self.pc += 4;
            }
            Opcodes::Input => {
                let input = self.inputs[self.input_idx];
                self.input_idx += 1;
                let out_addr = self.data[self.pc + 1] as usize;
                self.data[out_addr] = input;
                self.pc += 2;
            }
            Opcodes::Output => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                self.outputs.push(in1);
                self.pc += 2;
            }
            Opcodes::JumpIfTrue => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                if in1 != 0 {
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Opcodes::JumpIfFalse => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                if in1 == 0 {
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Opcodes::LessThan => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                let out_addr = self.data[self.pc + 3] as usize;
                self.data[out_addr] = if in1 < in2 { 1 } else { 0 };
                self.pc += 4;
            }
            Opcodes::Equals => {
                let in1 = self.get_val(self.data[self.pc + 1], param1_mode);
                let in2 = self.get_val(self.data[self.pc + 2], param2_mode);
                let out_addr = self.data[self.pc + 3] as usize;
                self.data[out_addr] = if in1 == in2 { 1 } else { 0 };
                self.pc += 4;
            }
            Opcodes::Halt => return false,
        }
        true
    }

    pub fn run(&mut self) {
        while self.step() {}
    }
}