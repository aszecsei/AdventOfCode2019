use num_enum::TryFromPrimitive;
use std::collections::HashMap;
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
    RelativeBaseOffset = 9,
    Halt = 99,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive)]
pub enum ParameterModes {
    Position = 0,
    Immediate = 1,
    Relative = 2,
}

fn get_num(digits: &[u8]) -> u8 {
    let mut acc = 0u8;
    for d in digits {
        acc *= 10;
        acc += *d as u8;
    }
    acc
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IntcodeStepResult {
    Ok,
    Halt,
    WaitingForInput,
}

pub struct Program {
    pub data: Vec<i64>,
    pub memory: HashMap<usize, i64>,
    pub pc: usize,
    pub inputs: Vec<i64>,
    pub outputs: Vec<i64>,
    status: IntcodeStepResult,
    input_idx: usize,
    relative_base: usize,
}

impl Program {
    pub fn new(data: &[i64], inputs: &[i64]) -> Self {
        Program {
            data: data.to_vec(),
            memory: HashMap::default(),
            pc: 0,
            inputs: inputs.to_vec(),
            outputs: vec![],
            status: IntcodeStepResult::Ok,
            input_idx: 0,
            relative_base: 0,
        }
    }

    fn get_val(&self, idx: i64, mode: ParameterModes) -> i64 {
        match mode {
            ParameterModes::Immediate => idx,
            ParameterModes::Position => self[idx as usize],
            ParameterModes::Relative => self[self.relative_base + idx as usize],
        }
    }

    fn get_val_mut(&mut self, idx: i64, mode: ParameterModes) -> &mut i64 {
        let rb = self.relative_base;
        match mode {
            ParameterModes::Immediate => panic!("Immediate mode cannot be used for outputs!"),
            ParameterModes::Position => &mut self[idx as usize],
            ParameterModes::Relative => &mut self[rb + idx as usize],
        }
    }

    pub fn step(&mut self) -> IntcodeStepResult {
        if self.status == IntcodeStepResult::Halt {
            return self.status;
        }

        let mut instruction = self[self.pc];
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
        let param3_mode: ParameterModes = digits[0].try_into().unwrap();

        match opcode {
            Opcodes::Addition => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                let out = self.get_val_mut(self[self.pc + 3], param3_mode);
                *out = in1 + in2;
                self.pc += 4;
            }
            Opcodes::Multiplication => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                let out = self.get_val_mut(self[self.pc + 3], param3_mode);
                *out = in1 * in2;
                self.pc += 4;
            }
            Opcodes::Input => {
                if self.input_idx >= self.inputs.len() {
                    self.status = IntcodeStepResult::WaitingForInput;
                    return self.status;
                }
                let input = self.inputs[self.input_idx];
                self.input_idx += 1;
                let out = self.get_val_mut(self[self.pc + 1], param1_mode);
                *out = input;
                self.pc += 2;
            }
            Opcodes::Output => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                self.outputs.push(in1);
                self.pc += 2;
            }
            Opcodes::JumpIfTrue => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                if in1 != 0 {
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Opcodes::JumpIfFalse => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                if in1 == 0 {
                    self.pc = in2 as usize;
                } else {
                    self.pc += 3;
                }
            }
            Opcodes::LessThan => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                let out = self.get_val_mut(self[self.pc + 3], param3_mode);
                *out = if in1 < in2 { 1 } else { 0 };
                self.pc += 4;
            }
            Opcodes::Equals => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                let in2 = self.get_val(self[self.pc + 2], param2_mode);
                let out = self.get_val_mut(self[self.pc + 3], param3_mode);
                *out = if in1 == in2 { 1 } else { 0 };
                self.pc += 4;
            }
            Opcodes::RelativeBaseOffset => {
                let in1 = self.get_val(self[self.pc + 1], param1_mode);
                self.relative_base += in1 as usize;
                self.pc += 2;
            }
            Opcodes::Halt => {
                self.status = IntcodeStepResult::Halt;
                return self.status;
            }
        }
        self.status = IntcodeStepResult::Ok;
        self.status
    }

    pub fn run(&mut self) {
        while self.step() == IntcodeStepResult::Ok {}
    }

    pub fn add_input(&mut self, input: i64) {
        self.inputs.push(input);
    }

    pub fn get_status(&self) -> IntcodeStepResult {
        self.status
    }
}

impl std::ops::Index<usize> for Program {
    type Output = i64;

    fn index(&self, idx: usize) -> &Self::Output {
        if idx < self.data.len() {
            &self.data[idx]
        } else if self.memory.contains_key(&idx) {
            self.memory.get(&idx).unwrap()
        } else {
            &0
        }
    }
}

impl std::ops::IndexMut<usize> for Program {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        if idx < self.data.len() {
            &mut self.data[idx]
        } else {
            self.memory.entry(idx).or_insert(0);
            self.memory.get_mut(&idx).unwrap()
        }
    }
}
