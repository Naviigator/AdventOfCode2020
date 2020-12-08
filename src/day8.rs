use std::collections::HashSet;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(8, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut count = 0;
    let mut index : i32 = 0;
    let mut ran_instructions : HashSet<i32> = HashSet::new();
    while  index >= 0 && (index as usize) < parser.instructions.len() {
        if ran_instructions.contains(&index) {
            break;
        }
        ran_instructions.insert(index);
        index += parser.instructions.get(index as usize).expect("there should be an instruction here").exec(&mut count);
    }

    println!("the count is {}", count);
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut checked_or_swapped_instructions : HashSet<i32> = HashSet::new();
    let mut count = 0;
    let mut index : i32 = 0;

    while (index as usize) != parser.instructions.len() {
        count = 0;
        index = 0;
        let mut swapped = false;
        let mut ran_instructions : HashSet<i32> = HashSet::new();
        while  index >= 0 && (index as usize) < parser.instructions.len() {
            let mut to_swap = false;
            if !swapped && !checked_or_swapped_instructions.contains(&index) {
                let instruction = parser.instructions.get(index as usize).expect("there should be an instruction here");
                if instruction.operation != "acc" {
                    swapped = true;
                    to_swap = true;
                }
                checked_or_swapped_instructions.insert(index);
            }
            if ran_instructions.contains(&index) {
                break;
            }
            ran_instructions.insert(index);
            index += parser.instructions.get(index as usize).expect("there should be an instruction here").exec_with_maybe_swap(to_swap, &mut count);
        }
    }

    println!("the count is {}", count);
}

struct Instruction {
    operation: String,
    amount: i32,
}
impl Instruction {
    fn new(operation: String, amount: i32) -> Instruction {
        Instruction {
            operation: operation,
            amount: amount,
        }
    }

    fn exec(&self, count : &mut i32) -> i32 {
        Instruction::exec_internal(self.operation.as_str(), self.amount, count)
    }

    fn exec_internal(operation: &str, amount : i32, count : &mut i32) -> i32 {
        match operation {
            "nop" => 1,
            "acc" => {
                *count += amount;
                1
            },
            "jmp" => amount,
            _ => panic!("unknown instruction"),
        }
    }

    fn exec_with_maybe_swap(&self, to_swap : bool, count : &mut i32) -> i32 {
        let mut operation = self.operation.as_str();
        if to_swap {
            operation = match operation {
                "nop" => "jmp",
                "jmp" => "nop",
                _ => panic!("WRONG!"),
            }
        }
        Instruction::exec_internal(operation, self.amount, count)
    }
}

struct Parser {
    instructions : Vec<Instruction>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            instructions: Vec::new(),
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let split : Vec<&str> = line.split(" ").collect();
        let operation = split.get(0).expect("operation expected");
        let amount = split.get(1).expect("content expected").parse().expect("should be a number");

        self.instructions.push(Instruction::new(operation.to_string(), amount));
    }
}
