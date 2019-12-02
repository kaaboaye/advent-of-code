use std::fs::read_to_string;

const ADDITION: usize = 1;
const MULTIPLICATION: usize = 2;
const EXIT: usize = 99;

const EXPECTED_RESULT: usize = 19690720;

fn main() {
    let initial_stack: Vec<usize> = load_stack("input.txt");

    for x in 0..99usize {
        for y in 0..99usize {
            let mut stack_ptr = 0usize;
            let mut stack = initial_stack.clone();


            // restore the state from before the fire
            stack[1] = x;
            stack[2] = y;

            while stack[stack_ptr] != EXIT {
                match stack[stack_ptr] {
                    ADDITION => {
                        let (a, b, res_ptr) = load_operation_registers(&stack, &stack_ptr);
                        stack[res_ptr] = a + b;
                    }
                    MULTIPLICATION => {
                        let (a, b, res_ptr) = load_operation_registers(&stack, &stack_ptr);
                        stack[res_ptr] = a * b;
                    }
                    _ => {
                        println!("Something went wrong\nStack ptr: {}", &stack_ptr);
                        println!("Program finished with the following stack\n{:?}", &stack);
                        break;
                    }
                };

                stack_ptr += 4;
            }


            if stack[0] == EXPECTED_RESULT {
                println!("Found for {} {}", x, y);
                println!("Final result {}", 100 * x + y);
                return;
            }
        }
    }

}

fn load_stack(path: &str) -> Vec<usize> {
    read_to_string(path)
        .unwrap()
        .split(",")
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn load_operation_registers(stack: &Vec<usize>, stack_ptr: &usize) -> (usize, usize, usize) {
    let a_ptr = stack[stack_ptr + 1];
    let a = stack[a_ptr];

    let b_ptr = stack[stack_ptr + 2];
    let b = stack[b_ptr];

    let res_ptr = stack[stack_ptr + 3];

    return (a, b, res_ptr);
}