use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(13, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut optimal_id = 0;
    let mut optimal_time_diff = 999999999;

    for test_time in parser.bus_times {
        let multiplier = parser.my_time / test_time;
        let next_bus_time = test_time * (multiplier + 1);
        if optimal_time_diff > next_bus_time - parser.my_time {
            optimal_time_diff = next_bus_time - parser.my_time;
            optimal_id = test_time;
        }
    }
    println!("let's take bus: {}, result is: {}", optimal_id, optimal_id * optimal_time_diff);
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    
    let mut to_increment_by = parser.bus_times_including_x[0];

    let mut last_valid_stop : HashMap<usize, i64> = HashMap::new();
    let mut indexes_to_check : Vec<usize> = (1..parser.bus_times_including_x.len()).collect();
    let mut indexes_to_remove : Vec<usize> = vec![];

    let mut timestamp : i64 = 0;
    let mut valid = false;
    while !valid {
        valid = true;
        timestamp += to_increment_by;
        for i in indexes_to_remove {
            indexes_to_check.retain( |x| *x != i);
        }
        indexes_to_remove = vec![];
        for i_ptr in &indexes_to_check {
            let i = *i_ptr;
            if parser.bus_times_including_x[i] == -1 {
                indexes_to_remove.push(i);
                continue;
            }
            let time_to_get = timestamp + (i as i64);
            if time_to_get % parser.bus_times_including_x[i] != 0 {
                valid = false;
                break;
            }
            if last_valid_stop.contains_key(&i) {
                to_increment_by = timestamp - last_valid_stop[&i];
                indexes_to_remove.push(i);
            } else {
                last_valid_stop.insert(i, timestamp);
            }
        }
    }
    println!("departure at: {}", timestamp);
}

struct Parser {
    my_time: i64,
    bus_times: Vec<i64>,
    bus_times_including_x: Vec<i64>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            my_time: -1,
            bus_times: vec![],
            bus_times_including_x: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        if self.my_time == -1 {
            self.my_time = line.parse().expect("Should have a starting time");
            return;
        }
        for bus_number in line.split(',') {
            if bus_number == "x" {
                self.bus_times_including_x.push(-1);
                continue;
            }
            self.bus_times.push(bus_number.parse().expect("not a number..."));
            self.bus_times_including_x.push(bus_number.parse().expect("not a number..."));
        }
    }
}