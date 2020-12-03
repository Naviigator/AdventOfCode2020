use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(3, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut trees_found = 0;
    let mut x = 0;
    for y in 1..parser.get_height() {
        x += 3;
        if parser.get_result(x, y) == '#' {
            trees_found += 1;
        }
    }
    println!("{} trees bumped into", trees_found)
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let moves: Vec<Coordinates> = vec![
        Coordinates { x: 1, y: 1 },
        Coordinates { x: 3, y: 1 },
        Coordinates { x: 5, y: 1 },
        Coordinates { x: 7, y: 1 },
        Coordinates { x: 1, y: 2 },
    ];

    let mut all_trees_found: Vec<i64> = vec![];
    for move_to_make in moves {
        let mut x = 0;
        let mut y = 0;
        let mut trees_found : i64 = 0;
        while y < parser.get_height() {
            if parser.get_result(x, y) == '#' {
                trees_found += 1;
            }
            x += move_to_make.x;
            y += move_to_make.y;
        }
        all_trees_found.push(trees_found);
    }
    let mut result: i64 = 1;
    for tree_count in all_trees_found {
        result *= tree_count;
    }
    println!("{} is the tree result", result)
}

#[derive(Hash, Eq, PartialEq, Debug)]
struct Coordinates {
    x: usize,
    y: usize,
}

struct Parser {
    map: HashMap<Coordinates, char>,
    current_y: usize,
    width: usize,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            map: HashMap::new(),
            current_y: 0,
            width: 0,
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let mut current_x = 0;
        for c in line.chars() {
            self.map.insert(
                Coordinates {
                    x: current_x,
                    y: self.current_y,
                },
                c,
            );
            current_x += 1;
        }
        self.width = current_x;

        self.current_y += 1;
    }

    fn get_result(&self, x: usize, y: usize) -> char {
        *self
            .map
            .get(&Coordinates {
                x: x % self.get_width(),
                y: y % self.get_height(),
            })
            .expect("the universe is a lie")
    }

    fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.current_y
    }
}
