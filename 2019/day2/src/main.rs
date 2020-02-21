use crate::vm::VM;

mod vm;


fn main() {
    let _res = VM::new("input.txt").run();

//    let initial_stack: Vec<usize> = load_memory("input.txt");
//
//    for x in 0..99usize {
//        for y in 0..99usize {
//            let mut program_counter = 0usize;
//            let mut memory = initial_stack.clone();
//
//
//            // restore the state from before the fire
//            memory[1] = x;
//            memory[2] = y;
//
//            while memory[program_counter] != EXIT {
//                match memory[program_counter] {
//                    ADDITION => {
//                        let (a, b, res_ptr) = load_operation_registers(&memory, &program_counter);
//                        memory[res_ptr] = a + b;
//                    }
//                    MULTIPLICATION => {
//                        let (a, b, res_ptr) = load_operation_registers(&memory, &program_counter);
//                        memory[res_ptr] = a * b;
//                    }
//                    _ => {
//                        println!("Something went wrong\nStack ptr: {}", &program_counter);
//                        println!("Program finished with the following stack\n{:?}", &memory);
//                        break;
//                    }
//                };
//
//                program_counter += 4;
//            }
//
//
//            if memory[0] == EXPECTED_RESULT {
//                println!("Found for {} {}", x, y);
//                println!("Final result {}", 100 * x + y);
//                return;
//            }
//        }
//    }
}

//fn load_memory(path: &str) -> Vec<usize> {
//    read_to_string(path)
//        .unwrap()
//        .split(",")
//        .map(|val| val.parse::<usize>().unwrap())
//        .collect::<Vec<usize>>()
//}