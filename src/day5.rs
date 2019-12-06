use std::io::{stdin, Error as IoError, ErrorKind, Read};

pub fn run() {
    let mut program = read_memory();
    run_computer(&mut program);
}

fn read_memory() -> Vec<i64> {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");

    let input_string = std::str::from_utf8(&s).expect("invalid utf8");
    let memory_values: Vec<i64> = input_string
        .split(",")
        .filter_map(|value| value.parse::<i64>().ok())
        .collect();
    memory_values
}

fn run_computer(mut memory: &mut Vec<i64>) {
    let mut program_counter = 0;
    loop {
        match perform_operation(&mut program_counter, &mut memory) {
            Ok(()) => (),
            Err(e) => {
                println!("{:?}", e);
                break;
            }
        }
    }
}

fn load(address: usize, memory: &Vec<i64>, mode: AddressingMode) -> Result<i64, IoError> {
    match mode {
        AddressingMode::Indirect => memory
            .get(
                memory
                    .get(address)
                    .ok_or(IoError::new(ErrorKind::Other, "failed load"))?
                    .clone() as usize,
            )
            .copied()
            .ok_or(IoError::new(ErrorKind::Other, "failed load")),
        AddressingMode::Immediate => memory
            .get(address)
            .copied()
            .ok_or(IoError::new(ErrorKind::Other, "failed load")),
    }
}

fn perform_operation(program_counter: &mut usize, memory: &mut Vec<i64>) -> Result<(), IoError> {
    let operation = parse_opcode(
        memory
            .get(*program_counter)
            .ok_or(IoError::new(ErrorKind::Other, "pc overflow"))?
            .clone(),
    );
    match operation {
        Some(Operation::Addition {
            summand1_mode: s1,
            summand2_mode: s2,
        }) => {
            let summand1 = load(*program_counter + 1, &memory, s1)?;
            let summand2 = load(*program_counter + 2, &memory, s2)?;
            let destination =
                load(*program_counter + 3, &memory, AddressingMode::Immediate)? as usize;
            memory[destination] = summand1 + summand2;
            // println!(
            //     "{:?} {} + {} -> [{}]",
            //     operation, summand1, summand2, destination
            // );
            *program_counter = *program_counter + 4;
        }
        Some(Operation::Multiplication {
            factor1_mode: f1,
            factor2_mode: f2,
        }) => {
            let factor1 = load(*program_counter + 1, &memory, f1)?;
            let factor2 = load(*program_counter + 2, &memory, f2)?;
            let destination =
                load(*program_counter + 3, &memory, AddressingMode::Immediate)? as usize;
            memory[destination] = factor1 * factor2;
            // println!(
            //     "{:?} {} + {} -> [{}]",
            //     operation, factor, summand2, destination
            // );
            *program_counter = *program_counter + 4;
        }
        Some(Operation::JumpIfTrue {
            condition_mode,
            value_mode,
        }) => {
            let condition = load(*program_counter + 1, &memory, condition_mode)?;
            let value = load(*program_counter + 2, &memory, value_mode)? as usize;
            *program_counter = if condition == 0 {
                *program_counter + 3
            } else {
                value
            }
        }
        Some(Operation::JumpIfFalse {
            condition_mode,
            value_mode,
        }) => {
            let condition = load(*program_counter + 1, &memory, condition_mode)?;
            let value = load(*program_counter + 2, &memory, value_mode)? as usize;
            *program_counter = if condition != 0 {
                *program_counter + 3
            } else {
                value
            }
        }
        Some(Operation::LessThan {
            left_parameter_mode: s1,
            right_parameter_mode: s2,
        }) => {
            let left_parameter = load(*program_counter + 1, &memory, s1)?;
            let right_parameter = load(*program_counter + 2, &memory, s2)?;
            let destination =
                load(*program_counter + 3, &memory, AddressingMode::Immediate)? as usize;
            memory[destination] = if left_parameter < right_parameter {
                1
            } else {
                0
            };
            *program_counter = *program_counter + 4;
        }
        Some(Operation::Equals {
            left_parameter_mode: s1,
            right_parameter_mode: s2,
        }) => {
            let left_parameter = load(*program_counter + 1, &memory, s1)?;
            let right_parameter = load(*program_counter + 2, &memory, s2)?;
            let destination =
                load(*program_counter + 3, &memory, AddressingMode::Immediate)? as usize;
            memory[destination] = if left_parameter == right_parameter {
                1
            } else {
                0
            };
            *program_counter = *program_counter + 4;
        }
        Some(Operation::Input) => {
            let pos =
                load(*program_counter + 1, &memory, AddressingMode::Immediate)?.clone() as usize;
            let mut arg_string = String::new();
            stdin().read_line(&mut arg_string)?;
            // let arg = arg_string
            //     .parse::<i64>()
            //     .map_err(|e| IoError::new(ErrorKind::Other, e.description()))?;
            // change to 1 for part 1 of day 5
            let arg = 5;
            memory[pos] = arg;
            *program_counter = *program_counter + 2;
        }
        Some(Operation::Print) => {
            let pos =
                load(*program_counter + 1, &memory, AddressingMode::Immediate)?.clone() as usize;
            println!("{}", memory[pos]);
            *program_counter = *program_counter + 2;
        }
        Some(Operation::Exit) => std::process::exit(0),
        _ => {
            return Err(IoError::new(
                ErrorKind::Other,
                format!(
                    "invalid opcode: {:?} at pc {}",
                    memory.get(*program_counter),
                    *program_counter
                ),
            ))
        }
    };
    Ok(())
}

fn parse_opcode(opcode: i64) -> Option<Operation> {
    let operation_str = opcode % 100;
    let op1_mode_str = (opcode / 100) % 10;
    let op2_mode_str = (opcode / 1000) % 10;

    match operation_str {
        1 => Some(Operation::Addition {
            summand1_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            summand2_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        2 => Some(Operation::Multiplication {
            factor1_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            factor2_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        3 => Some(Operation::Input),
        4 => Some(Operation::Print),
        5 => Some(Operation::JumpIfTrue {
            condition_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            value_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        6 => Some(Operation::JumpIfFalse {
            condition_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            value_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        7 => Some(Operation::LessThan {
            left_parameter_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            right_parameter_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        8 => Some(Operation::Equals {
            left_parameter_mode: mode_str_to_addressing_mode(op1_mode_str)?,
            right_parameter_mode: mode_str_to_addressing_mode(op2_mode_str)?,
        }),
        99 => Some(Operation::Exit),
        _ => None,
    }
}

fn mode_str_to_addressing_mode(c: i64) -> Option<AddressingMode> {
    match c {
        0 => Some(AddressingMode::Indirect),
        1 => Some(AddressingMode::Immediate),
        _ => None,
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Operation {
    Addition {
        summand1_mode: AddressingMode,
        summand2_mode: AddressingMode,
    },
    Multiplication {
        factor1_mode: AddressingMode,
        factor2_mode: AddressingMode,
    },
    Input,
    Print,
    JumpIfTrue {
        condition_mode: AddressingMode,
        value_mode: AddressingMode,
    },
    JumpIfFalse {
        condition_mode: AddressingMode,
        value_mode: AddressingMode,
    },
    LessThan {
        left_parameter_mode: AddressingMode,
        right_parameter_mode: AddressingMode,
    },
    Equals {
        left_parameter_mode: AddressingMode,
        right_parameter_mode: AddressingMode,
    },
    Exit,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum AddressingMode {
    Indirect,
    Immediate,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_opcode_example1() {
        assert_eq!(
            parse_opcode(1002),
            Some(Operation::Multiplication {
                factor1_mode: AddressingMode::Indirect,
                factor2_mode: AddressingMode::Immediate
            })
        );
    }
}
