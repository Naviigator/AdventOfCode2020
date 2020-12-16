use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(16, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut wrong_ones : i64 = 0;

    let possibilities = parser.possibilities;
    
    for ticket in parser.tickets {
        for number in ticket {
            if get_number_types(number, &possibilities).is_empty() {
                wrong_ones += number;
            }
        }
    }
    println!("{} is the sum of wrong numbers", wrong_ones);
}

pub fn get_number_types(number : i64, possibilities : &HashMap<String, HashSet<i64>>) -> Vec<&str> {
    let mut result : Vec<&str> = vec![];
    for (key, legit_values) in possibilities {
        if legit_values.contains(&number) {
            result.push(key);
        }
    }
    result
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let possibilities = &parser.possibilities;

    let mut all_keys = vec![];
    for (key, _val) in &parser.possibilities {
        all_keys.push(key.clone());
    }

    let mut valid_values : Vec<Vec<String>> = vec![];
    for _i in 0..parser.your_ticket.len() {
        valid_values.push(all_keys.clone());
    }
    
    valid_values = get_ticket_values(&parser.your_ticket, possibilities, &valid_values);

    for ticket in parser.tickets {
        valid_values = get_ticket_values(&ticket, possibilities, &valid_values);
    }

    let mut new_valid_values = vec![];
    loop {
        let mut values_to_remove_from_where_not_only_value = vec![];
        let mut value_counts : HashMap<String, usize> = HashMap::new();

        for possibilities in &valid_values {
            if possibilities.len() == 1 {
                values_to_remove_from_where_not_only_value.push(possibilities[0].clone());
            } else {
                for possibility in possibilities {
                    let possible_value = value_counts.get(possibility);
                    let mut count = 1;
                    if possible_value.is_some() {
                        count = possible_value.unwrap() + 1;
                    }
                    value_counts.insert(possibility.clone(), count);
                }
            }
        }

        let mut values_that_are_unique = vec![];
        for (k, v) in value_counts {
            if v == 1 {
                values_that_are_unique.push(k);
            }
        }

        for possibilities in &valid_values {
            if possibilities.len() == 1 {
                new_valid_values.push(possibilities.clone());
                continue;
            }
            let mut unique_found = false;
            for unique_possibility in &values_that_are_unique {
                if possibilities.contains(&unique_possibility) {
                    new_valid_values.push(vec![unique_possibility.clone()]);
                    unique_found = true;
                    break;
                }
            }
            if unique_found {
                continue;
            }

            let new_possibilities = possibilities.iter().filter(|&x| !values_to_remove_from_where_not_only_value.contains(x)).cloned().collect();
            new_valid_values.push(new_possibilities);
        }

        if new_valid_values == valid_values {
            break;
        }

        valid_values = new_valid_values.clone();
        new_valid_values = vec![];
    }
    let mut final_value = 1;
    for i in 0..valid_values.len() {
        let singleton_value = &valid_values[i];
        if singleton_value.len() != 1 {
            panic!("there should only be 1 result");
        }
        let value = &singleton_value[0];
        if value.starts_with("departure") {
            final_value *= parser.your_ticket[i];
        }
    }
    println!("{:?} is the result", final_value);
}

fn get_ticket_values(ticket : &Vec<i64>, possibilities : &HashMap<String, HashSet<i64>>, keys: &Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut new_valid_values = vec![];
    let mut valid = true;
    for i in 0..ticket.len() {
        let number = ticket[i];
        let valid_for_this_number = get_number_types_for(number, possibilities, &keys[i]);
        if valid_for_this_number.is_empty() {
            valid = false;
            break;
        }
        new_valid_values.push(valid_for_this_number);
    }
    if valid {
        return new_valid_values;
    }
    keys.clone()
}

fn get_number_types_for(number : i64, possibilities : &HashMap<String, HashSet<i64>>, keys: &Vec<String>) -> Vec<String> {
    let mut result : Vec<String> = vec![];
    for key in keys {
        if possibilities.get(key).unwrap().contains(&number) {
            result.push(key.clone());
        }
    }
    result
}

struct Parser {
    possibilities: HashMap<String, HashSet<i64>>,
    tickets: Vec<Vec<i64>>,
    your_ticket: Vec<i64>,
    mode: String,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            possibilities: HashMap::new(),
            tickets: vec![],
            your_ticket: vec![],
            mode: "".to_string(),
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        if line == "your ticket:" || line == "nearby tickets:" {
            self.mode = line;
            return;
        }
        if self.mode == "".to_string() {
            let split_on_semi : Vec<&str> = line.split(": ").collect();
            let name  = split_on_semi[0];
            let ranges : Vec<&str> = split_on_semi[1].split(" or ").collect();
            for range in ranges {
                let split : Vec<&str> = range.split("-").collect();
                let mut set : HashSet<i64> = HashSet::new();
                if self.possibilities.contains_key(name) {
                    set.extend(self.possibilities.get(name).unwrap().clone());
                }
                let from : i64 = split[0].parse().unwrap();
                let to : i64 = split[1].parse().unwrap();
                let new_content = from..=to;
                set.extend(new_content);
                self.possibilities.insert(name.to_string(), set);
            }
        } else {
            let numbers : Vec<i64> = line.split(",").map(|x| x.parse().unwrap()).collect();

            if self.mode == "your ticket:".to_string() {
                self.your_ticket = numbers;
            } else {
                self.tickets.push(numbers);
            }
        }
    }
}