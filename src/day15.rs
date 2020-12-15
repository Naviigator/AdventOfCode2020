use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(15, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    
    run_until(&parser.numbers, 2020);
}

fn run_until(numbers : &Vec<i64>, limit : i64) {
    let mut index = 0;
    let mut last_index : HashMap<i64, i64> = HashMap::new();
    let mut current_number = -1;
    for number in numbers {
        if current_number != -1 {
            last_index.insert(current_number, index);
        }
        index += 1;
        current_number = *number;
    }
    while index < limit {
        let last_found_at = last_index.get(&current_number);
        let mut new_number = 0;
        if last_found_at.is_some() {
            new_number = index - *last_found_at.expect("it was some!");
        }

        last_index.insert(current_number, index);
        index += 1;
        current_number = new_number;
    }
    println!("{} is the number", current_number);
}


pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    
    
    run_until(&parser.numbers, 30000000);
}

struct Parser {
    numbers: Vec<i64>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            numbers: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let numbers_as_string : Vec<&str> = line.split(",").collect();
        for nbr_str in numbers_as_string {
            self.numbers.push(nbr_str.parse().expect("should be a number"));
        }
    }
}