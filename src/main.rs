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
    EXP,

    LT,
    GT,
    EQ,
    AND,
    OR,
    NOT,

    JUMP(String),

    POP,
    PUSH(i32),
}

impl FromStr for OpCode {
    type Err = ();
    fn from_str(input: &str) -> Result<OpCode, Self::Err> {
        let codes: Vec<&str> = input.split_whitespace().collect();
        let mut value = 0;
        let mut function: String = "".to_owned();

        if (codes.len() > 1) & (codes[0] == "PUSH") {
            value = codes[1].parse::<i32>().unwrap();
        } else if (codes.len() > 1) & (codes[0] == "JUMP") {
            function = codes[1].to_owned();
        }

        match codes[0] {
            "ADD" => Ok(OpCode::ADD),
            "SUB" => Ok(OpCode::SUB),
            "MUL" => Ok(OpCode::MUL),
            "DIV" => Ok(OpCode::DIV),
            "MOD" => Ok(OpCode::MOD),
            "EXP" => Ok(OpCode::EXP),

            "LT" => Ok(OpCode::LT),
            "GT" => Ok(OpCode::GT),
            "EQ" => Ok(OpCode::EQ),
            "AND" => Ok(OpCode::AND),
            "OR" => Ok(OpCode::OR),
            "NOT" => Ok(OpCode::NOT),

            "JUMP" => Ok(OpCode::JUMP(function)),

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

    // Define pointers
    let mut stackpointer: usize = 0;
    let mut programpointer: usize = 0;

    while programpointer < programs.len() {
        let program = &programs[programpointer];
        println!("{}", programpointer);
        programpointer += 1;
        let p = OpCode::from_str(program).unwrap();
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

            OpCode::EXP => {
                let rt = exp(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::LT => {
                let rt = lt(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::GT => {
                let rt = gt(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::EQ => {
                let rt = eq(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::AND => {
                let rt = and(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::OR => {
                let rt = or(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::NOT => {
                let rt = not(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }

            OpCode::JUMP(function) => {
                programpointer = jump(function, programs.clone());
            }

            OpCode::POP => {
                let rt = pop(stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.2;
            }

            OpCode::PUSH(value) => {
                let rt = push(value, stack, stackpointer);
                stack = rt.0;
                stackpointer = rt.1;
            }
        }
        println!("{:?}", stack);
    }

    println!("{:?}", stack);
}

// define a match statement that makes the reactions for each opcode

// Basic operations
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

fn exp(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push(a.pow(b.try_into().unwrap()), rt_b.0, stackpointer);
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

// Boolean operations
fn lt(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push((a < b) as i32, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn gt(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push((a > b) as i32, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn eq(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push((a == b) as i32, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn and(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push((a & b) as i32, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn or(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt_b = pop(rt_a.0, stackpointer);
    let b: i32 = rt_b.1;
    stackpointer = rt_b.2;

    let rt = push((a | b) as i32, rt_b.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

fn not(mut stack: Vec<i32>, mut stackpointer: usize) -> (Vec<i32>, usize) {
    let rt_a = pop(stack, stackpointer);
    let a = rt_a.1;
    stackpointer = rt_a.2;

    let rt = push(!a as i32, rt_a.0, stackpointer);
    stack = rt.0;
    stackpointer = rt.1;

    return (stack, stackpointer);
}

// Advanced operations
// if_else
// als waar
// dan jump naar if
// anders jump naar else

fn jump(function: String, programs: Vec<String>) -> usize {
    //search in program

    let new_pointer: usize = programs.iter().position(|p| *p == function).unwrap();

    return new_pointer;
}

// Adjusting the stack operations
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
