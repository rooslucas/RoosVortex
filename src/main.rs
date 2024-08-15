// define an ENUM with op codes
enum OpCode {
    ADD,
    SUB,
    MUL,
    DIV,
    POP,
    PUSH,
}

// Make a program list

// Make a stack

// make pointers
fn main() {
    let stack: Vec<i32> = vec![0, 100]; // make it a vector of all 0's
    let program: Vec<OpCode> = vec![];
    let stackpointer: i32 = 0;
}

// define a match statement that makes the reactions for each opcode

fn match_program(program: OpCode, stack: Vec<i32>, stackpointer: usize) {
    match program {
        OpCode::ADD => add(stack, stackpointer),
        OpCode::SUB => sub(stack, stackpointer),
        OpCode::MUL => mul(stack, stackpointer),
        OpCode::DIV => div(stack, stackpointer),
        OpCode::POP => println!("maybe later"), //pop(stack, stackpointer),
        OpCode::PUSH => println!("maybe later"), //push(value, stack, stackpointer),
    }
}

fn add(stack: Vec<i32>, stackpointer: usize) {
    let a = pop(stack.clone(), stackpointer);
    let b: i32 = pop(stack.clone(), stackpointer);
    push(a + b, stack.clone(), stackpointer);
}

fn sub(stack: Vec<i32>, stackpointer: usize) {
    let a = pop(stack.clone(), stackpointer);
    let b: i32 = pop(stack.clone(), stackpointer);
    push(a - b, stack.clone(), stackpointer);
}

fn mul(stack: Vec<i32>, stackpointer: usize) {
    let a = pop(stack.clone(), stackpointer);
    let b: i32 = pop(stack.clone(), stackpointer);
    push(a * b, stack.clone(), stackpointer)
}

fn div(stack: Vec<i32>, stackpointer: usize) {
    let a = pop(stack.clone(), stackpointer);
    let b: i32 = pop(stack.clone(), stackpointer);
    push(a / b, stack.clone(), stackpointer)
}

fn pop(mut stack: Vec<i32>, mut stackpointer: usize) -> i32 {
    if stackpointer > 0 {
        stackpointer -= 1;
    } else {
        stackpointer = stack.len() - 1;
    }

    let rt: i32 = stack[stackpointer];
    stack[stackpointer] = 0;
    return rt;
}

fn push(value: i32, mut stack: Vec<i32>, mut stackpointer: usize) {
    stack[stackpointer] = value;
    stackpointer += 1;
}
