use std::io::{stdin, Read};

pub fn run() {
    let program = read_memory();

    for i in 0..100 {
        for j in 0..100 {
            let mut memory = program.clone();
            memory[1] = i;
            memory[2] = j;
            run_computer(&mut memory);

            if memory[0] == 19690720 {
                println!("{}", 100 * memory[1] + memory[2]);
                break;
            }
        }
    }
}

fn read_memory() -> Vec<u64> {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");

    let input_string = std::str::from_utf8(&s).expect("invalid utf8");
    let memory_values: Vec<u64> = input_string
        .split(",")
        .filter_map(|value| value.parse::<u64>().ok())
        .collect();
    memory_values
}

fn run_computer(mut memory: &mut Vec<u64>) {
    let mut program_counter = 0;
    loop {
        match memory.get(program_counter) {
            Some(&value) => match value {
                1 => {
                    perform_operation(&mut program_counter, &mut memory, |a, b| a + b);
                }
                2 => {
                    perform_operation(&mut program_counter, &mut memory, |a, b| a * b);
                }
                99 => break,
                _ => break,
            },
            None => break,
        }
    }
}

fn perform_operation(
    program_counter: &mut usize,
    memory: &mut Vec<u64>,
    mut operation: impl FnMut(u64, u64) -> u64,
) {
    if let (Some(result_position), Some(add1_position), Some(add2_position)) = (
        memory
            .get(*program_counter + 3)
            .cloned()
            .map(|v| v as usize),
        memory
            .get(*program_counter + 1)
            .cloned()
            .map(|v| v as usize),
        memory
            .get(*program_counter + 2)
            .cloned()
            .map(|v| v as usize),
    ) {
        if let (Some(add1), Some(add2), Some(result)) = (
            memory.get(add1_position).cloned(),
            memory.get(add2_position).cloned(),
            memory.get_mut(result_position),
        ) {
            *result = operation(add1, add2);
        } else {
            panic!("invalid memory positions");
        }

        *program_counter = *program_counter + 4;
    } else {
        panic!("overflow at pc {}", program_counter);
    }
}
