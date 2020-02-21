mod integer;
mod operation_result;
mod pointer;
mod value_type;

use std::fs::read_to_string;
use crate::vm::operation_result::OperationResult;
use crate::vm::operation_result::OperationResult::{Exit, Error, Continue};
use crate::vm::value_type::ValueType;
use crate::vm::integer::Integer;
use crate::vm::pointer::Pointer;


pub const ADD: usize = 1;
pub const MUL: usize = 2;
pub const EXIT: usize = 99;

pub struct VM {
    memory: Vec<usize>,
    program_counter: usize,
}


impl VM {
    pub fn new(path: &str) -> VM {
        let memory = read_to_string(path)
            .unwrap()
            .split(",")
            .map(|val| val.parse::<usize>().unwrap())
            .collect::<Vec<usize>>();

        VM { memory, program_counter: 0 }
    }

    pub fn run(&mut self) -> Result<(), String> {
        match self.run_operation() {
            Continue => self.run(),
            Exit => Ok(()),
            Error(err) => Err(err)
        }
    }

    pub fn run_operation(&mut self) -> OperationResult {
        let Integer(operation) = self.pc_value();

        if operation == EXIT {
            return Exit;
        }

        match operation {
            ADD => self.add_opp(),
            MUL => self.mul_opp(),
            _ => Error(format!("Unknown operation: {}", operation))
        }
    }

    fn add_opp(&mut self) -> OperationResult {
        let (a, b, res_ptr) = self.load_tri_operation_args();
        let res = a + b;

        self.store_at(res, res_ptr);
        self.move_pc(4);

        Continue
    }

    fn mul_opp(&mut self) -> OperationResult {
        let (a, b, res_ptr) = self.load_tri_operation_args();
        let res = a * b;

        self.store_at(res, res_ptr);
        self.move_pc(4);

        Continue
    }

    fn load_tri_operation_args(&self) -> (Integer, Integer, Pointer) {
        let a_ptr: Pointer = self.pc_offset_value(1);
        let a: Integer = self.pointer_value(a_ptr);

        let b_ptr: Pointer = self.pc_offset_value(2);
        let b: Integer = self.pointer_value(b_ptr);

        let res_ptr: Pointer = self.pc_offset_value(3);

        return (a, b, res_ptr);
    }

    fn move_pc(&mut self, offset: usize) {
        self.program_counter += offset;
    }

    fn pc_value<T>(&self) -> T where T: ValueType {
        let val = self.memory[self.program_counter];
        T::new(val)
    }

    fn pc_offset_value<T>(&self, offset: usize) -> T where T: ValueType {
        let val = self.memory[self.program_counter + offset];
        T::new(val)
    }

    fn pointer_value<T>(&self, ptr: Pointer) -> T where T: ValueType {
        let Pointer(ptr) = ptr;
        let val = self.memory[ptr];
        T::new(val)
    }

    fn store_at<T>(&mut self, val: T, ptr: Pointer) where T: ValueType {
        let val = val.to_usize();
        let Pointer(ptr) = ptr;
        self.memory[ptr] = val;
    }
}

