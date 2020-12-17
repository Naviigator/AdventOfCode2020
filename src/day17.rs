use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(17, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    
    let mut puzzle = parser.input;

    for _i in 0..6 {
        puzzle = tick(puzzle);
    }
    println!("{} is the sum of wrong numbers", puzzle.len());
}

fn tick(puzzle : HashMap<Coordinate, bool>) -> HashMap<Coordinate, bool> {
    let mut heatmap: HashMap<Coordinate, i32> = HashMap::new();
    for (k, v) in &puzzle {
        if *v {
            for x in k.x - 1..=k.x+1 {
                for y in k.y - 1..=k.y+1 {
                    for z in k.z - 1..=k.z+1 {
                        if x == k.x && y == k.y && z == k.z {
                            continue;
                        }
                        let coord = Coordinate{ x: x, y: y, z: z };
                        let mut val = 0;
                        let maybe_coord = heatmap.get(&coord);
                        if maybe_coord.is_some() {
                            val = *maybe_coord.unwrap();
                        }
                        heatmap.insert(Coordinate{ x: x, y: y, z: z }, val + 1);
                    }
                }
            }
        }
    }

    let mut result : HashMap<Coordinate, bool> = HashMap::new();

    for (k, v) in heatmap {
        if v == 3 {
            result.insert(k, true);
            continue;
        }
        let maybe = puzzle.get(&k);
        if v == 2 && maybe.is_some() && *maybe.unwrap() {
            result.insert(k, true);
        }
    }
    result
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    
    let mut puzzle = convert_to_4(&parser.input);

    for _i in 0..6 {
        puzzle = tick2(puzzle);
    }
    println!("{} is the sum of wrong numbers", puzzle.len());
}

fn tick2(puzzle : HashMap<Coordinate4, bool>) -> HashMap<Coordinate4, bool> {
    let mut heatmap: HashMap<Coordinate4, i32> = HashMap::new();
    for (k, v) in &puzzle {
        if *v {
            for w in k.w - 1..=k.w+1 {
                for x in k.x - 1..=k.x+1 {
                    for y in k.y - 1..=k.y+1 {
                        for z in k.z - 1..=k.z+1 {
                            if w == k.w && x == k.x && y == k.y && z == k.z {
                                continue;
                            }
                            let coord = Coordinate4{ w: w, x: x, y: y, z: z };
                            let mut val = 0;
                            let maybe_coord = heatmap.get(&coord);
                            if maybe_coord.is_some() {
                                val = *maybe_coord.unwrap();
                            }
                            heatmap.insert(Coordinate4{ w: w, x: x, y: y, z: z }, val + 1);
                        }
                    }
                }
            }
        }
    }

    let mut result : HashMap<Coordinate4, bool> = HashMap::new();

    for (k, v) in heatmap {
        if v == 3 {
            result.insert(k, true);
            continue;
        }
        let maybe = puzzle.get(&k);
        if v == 2 && maybe.is_some() && *maybe.unwrap() {
            result.insert(k, true);
        }
    }
    result
}

fn convert_to_4(input : &HashMap<Coordinate, bool>) -> HashMap<Coordinate4, bool> {
    let mut result = HashMap::new();
    for (k, v) in input {
        result.insert(Coordinate4{w: 0, x: k.x, y: k.y, z: k.z}, *v);
    }
    result
}


#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinate {
    x: i64,
    y: i64,
    z: i64,
}
#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinate4 {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

struct Parser {
    input: HashMap<Coordinate, bool>,
    current_y: i64,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            input: HashMap::new(),
            current_y: 0,
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }

        let mut x = 0;
        for c in line.chars() {
            let mut to_insert = false;
            if c == '#' {
                to_insert = true;
            }
            self.input.insert(Coordinate{ x: x, y: self.current_y, z: 0 }, to_insert);
            x += 1;
        }
        self.current_y += 1;
    }
}