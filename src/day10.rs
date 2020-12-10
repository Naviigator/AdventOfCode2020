use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(10, sample_data, &mut |s: String| parser.parse_string(s));
    parser.done();
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut current_joltage = 0;
    let mut all_jolt_differences : HashMap<i64, i64> = HashMap::new();
    
    for spec in parser.charger_specs {
        let joltage_diff = spec - current_joltage;
        if joltage_diff > 3 {
            break;
        }
        *all_jolt_differences.entry(joltage_diff).or_insert(0) += 1;
        current_joltage = spec;
    }
    //one more for current device
    *all_jolt_differences.entry(3).or_insert(0) += 1;

    println!("{}", all_jolt_differences[&1] * all_jolt_differences[&3])
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    println!("{}", get_count(&parser.charger_specs))
}

fn get_count(input : &Vec<i64>) -> i64 {
    let mut paths_from : HashMap<i64, i64> = HashMap::new();

    let mut max_joltage = 0;
    let mut max_joltage_idx = 0;
    
    for i in 0..input.len() {
        let spec = *input.get(i).expect("joltage expected");
        let joltage_diff = spec - max_joltage;
        if joltage_diff > 3 {
            break;
        }
        max_joltage_idx = i;
        max_joltage = spec;
    }

    paths_from.insert(*input.get(max_joltage_idx).expect("need a number"), 1);

    for i in (0..max_joltage_idx).rev() {
        let mut paths = 0;
        let current = *input.get(i).expect("need a number");
        for j in i + 1..max_joltage_idx + 1 {
            let new = *input.get(j).expect("need a new number");
            if new - current > 3 {
                break;
            }
            
            paths += *paths_from.get(&new).expect("cache should be filled");
        }
        paths_from.insert(current, paths);
    }
    
    *paths_from.entry(1).or_default() + *paths_from.entry(2).or_default() + *paths_from.entry(3).or_default()
}

struct Parser {
    charger_specs : Vec<i64>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            charger_specs: Vec::new(),
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }

        self.charger_specs.push(line.parse().expect("should be a number"));
    }

    pub fn done(&mut self) {
        self.charger_specs.sort();
    }
}
