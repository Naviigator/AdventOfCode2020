use std::collections::HashSet;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(5, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    println!("{} is the max id", parser.seats.iter().map(|seat| seat.get_id()).max().expect("Why is there nothing?"));
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let id_set : HashSet<i32> = parser.seats.iter().map(|seat| seat.get_id()).collect();
    let min = *id_set.iter().min().expect("we need a minimum!");
    let max = *id_set.iter().max().expect("we need a maximum!");
    for i in (min + 1)..max {
        if !id_set.contains(&i) {
            println!("Your seat ID is {}, have a nice flight!", i);
            return
        }
    }
    println!("It seems there are no seats for you. No vacation it is!");
}

struct Seat {
    id: String,
}
impl Seat {
    fn new(id : String) -> Seat {
        Seat {
            id,
        }
    }

    fn get_row(&self) -> i32 {
        let mut min = 0;
        let mut max_excl = 128;
        for c in self.id.chars() {
            if c == 'F' {
                max_excl -= (max_excl - min) / 2
            } else if c == 'B' {
                min += (max_excl - min) / 2
            }
        }
        min
    }

    fn get_column(&self) -> i32 {
        let mut min = 0;
        let mut max_excl = 8;
        for c in self.id.chars() {
            if c == 'L' {
                max_excl -= (max_excl - min) / 2
            } else if c == 'R' {
                min += (max_excl - min) / 2
            }
        }
        min
    }

    fn get_id(&self) -> i32 {
        self.get_row() * 8 + self.get_column()
    }
}

struct Parser {
    seats : Vec<Seat>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            seats: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        self.seats.push(Seat::new(line));
    }
}
