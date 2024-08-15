use std::fs::read_to_string;
use std::str::FromStr;

// define an ENUM with op codes
#[derive(Debug, PartialEq)]
enum OpCode {
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    POP,
    PUSH(i32),
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        let codes: Vec<&str> = input.split_whitespace().collect();
        let mut value = 0;

        if codes.len() > 1 {
            value = codes[1].parse::<i32>().unwrap();
        }

        match codes[0] {
            "ADD" => Ok(OpCode::ADD),
            "SUB" => Ok(OpCode::SUB),
            "MUL" => Ok(OpCode::MUL),
            "DIV" => Ok(OpCode::DIV),
            "MOD" => Ok(OpCode::MOD),
            "POP" => Ok(OpCode::POP),
            "PUSH" => Ok(OpCode::PUSH(value)),
            _ => Err(()),
        }
    } // Match statement with opcode
      // if statement is push, get value out and put it in brackets
}

fn main() {
    // Make a stack
    let mut stack: Vec<i32> = vec![];

    // Read program instructions from file
    let mut programs = Vec::new();
    for line in read_to_string("./Input/input.rv").unwrap().lines() {
        programs.push(line.to_string());
    }

    // Define pointer
    let mut stackpointer: usize = 0;

    for program in programs {
        let p = OpCode::from_str(&program).unwrap();
        match p {
            OpCode::ADD => {
                let rt = add(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }
            OpCode::SUB => {
                let rt = sub(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }
            OpCode::MUL => {
                let rt = mul(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }
            OpCode::DIV => {
                let rt = div(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::MOD => {
                let rt = modu(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::POP => {
                let rt = pop(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.2;
            } //pop(stack, stackpointer),
            OpCode::PUSH(value) => {
                let rt = push(value, stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            } //push(value, stack, stackpointer),
        }
        println!("{:?}", stack);
    }

    println!("{:?}", stack);
}

// define a match statement that makes the reactions for each opcode

fn add(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a + b, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn sub(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a - b, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn mul(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a * b, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn div(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a / b, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn modu(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a % b, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn pop(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, i32, usize) {
    if stackpointer > 0 {
        stackpointer -= 1;
    } else {
        stackpointer = stack.len() - 1;
    }

    let rt: i32 = stack.pop().unwrap();
    return (stack, rt, stackpointer);
}

fn push(value: i32, mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    stack.push(value);
    stackpointer += 1;
    return (stack, stackpointer);
}
