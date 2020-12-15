use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(14, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut registry : HashMap<i64, i64> = HashMap::new();

    let mut or_mask = 0;
    let mut and_mask = !0;

    for command in parser.commands {
        if command.cmd == "mask" {
            or_mask = command.number_1;
            and_mask = command.number_2;
        } else {
            let value = command.number_2 & and_mask | or_mask;
            registry.insert(command.number_1, value);
        }
    }
    let result : i64 = registry.values().sum();
    println!("values are {}", result);
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut registry : HashMap<i64, i64> = HashMap::new();

    let mut or_mask = 0;
    let mut and_mask = 0;

    for _i in 0..36 {
        and_mask <<= 1;
        and_mask +=1;
    }

    for command in parser.commands {
        if command.cmd == "mask" {
            or_mask = command.number_1;
            and_mask = command.number_2;
        } else {
            store_value(command.number_1, command.number_2, &mut registry, or_mask, and_mask);
        }
    }
    let result : i64 = registry.values().sum();
    println!("values are {}", result);
}

fn store_value(address : i64, number_to_store : i64, registry : &mut HashMap<i64, i64>, or_mask : i64, and_mask : i64) {
    store_value_recursive(address, number_to_store, registry, or_mask, and_mask, 0);
}

fn store_value_recursive(address : i64, number_to_store : i64, registry : &mut HashMap<i64, i64>, or_mask : i64, and_mask : i64, depth : i64) {
    if depth == 36 {
        registry.insert(address, number_to_store);
        return;
    }
    let mut new_address = address;
    if is_floating(or_mask, and_mask, depth) {
        let mut mask = 1;
        mask <<= depth;
        mask = !mask;
        let temp_address = address & mask;
        store_value_recursive(temp_address, number_to_store, registry, or_mask, and_mask, depth + 1);
    }
    if is_overwritten(or_mask, depth) || is_floating(or_mask, and_mask, depth) {
        let mut mask = 1;
        mask <<= depth;
        new_address = address | mask;
    }
    store_value_recursive(new_address, number_to_store, registry, or_mask, and_mask, depth + 1);
}

fn is_floating(or_mask : i64, and_mask : i64, depth : i64) -> bool {
    let mut mask = 1;
    mask <<= depth;
    or_mask & mask == 0 && and_mask & mask != 0
}

fn is_overwritten(or_mask : i64, depth : i64) -> bool {
    let mut mask = 1;
    mask <<= depth;
    or_mask & mask != 0
}

struct Command {
    cmd: String,
    number_1: i64,//or mask || index
    number_2: i64,//and mask || value
}
impl Command {
    fn new(cmd: String, value: String) -> Command {
        let mut number_1 = 0;
        let mut number_2 = 0;
        if cmd == "mask" {
            for c in value.chars() {
                number_1 <<= 1;
                number_2 <<= 1;
                number_2 += 1;
                if c == '1' {
                    number_1 += 1;
                } else if c == '0' {
                    number_2 -= 1;
                }
            }
        } else {
            let mut values : Vec<&str> = cmd.split('[').collect();
            values = values[1].split(']').collect();
            number_1 = values[0].parse().expect("expected a number");
            number_2 = value.parse().expect("expected a number");
        }
        Command{
            cmd: cmd,
            number_1: number_1,
            number_2: number_2,
        }
    }
}

struct Parser {
    commands: Vec<Command>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            commands: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let data : Vec<&str> = line.split(" = ").collect();
        self.commands.push(Command::new(data[0].to_string(), data[1].to_string()));
    }
}