fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(11, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut puzzle = parser.map;

    let mut count = 0;
    let mut steady_state = false;
    while !steady_state {
        count += 1;
        steady_state = tick(&mut puzzle);
    }
    println!("{} with {} ticks", puzzle.iter()
    .map(|x : &Vec<char>| x.iter().filter(|y| **y == '#').count())
    .sum::<usize>(), count);
}

fn tick(input : &mut Vec<Vec<char>>) -> bool {
    let mut heatmap : Vec<Vec<i32>> = vec![vec![0; input.get(0).expect("one should be here").len()]; input.len()];
    let mut y = 0;
    for row in input.iter() {
        let mut x = 0;
        for c in row {
            if c == &'#' {
                mark_neighbours(&mut heatmap, x, y)
            }
            x += 1;
        }
        y += 1;
    }
    
    update_using_heatmap(input, &heatmap, 3)
}

fn mark_neighbours(heatmap : &mut Vec<Vec<i32>>, point_x: usize, point_y: usize) {
    let min_x = get_one_less_or_zero(point_x);
    let min_y = get_one_less_or_zero(point_y);
    for y in min_y..=point_y + 1 {
        for x in min_x..=point_x + 1 {
            if x == point_x && y == point_y {
                continue;
            }
            if let Some(val) = heatmap.get_mut(y).and_then(|row| row.get_mut(x)) {
                *val += 1;
            }
        }
    }
}

fn get_one_less_or_zero(x : usize) -> usize {
    let mut min_x = x;
    if min_x != 0 {
        min_x -= 1;
    }
    min_x
}

fn update_using_heatmap(input : &mut Vec<Vec<char>>, heatmap : &Vec<Vec<i32>>, max_heatmap_count : i32) -> bool {
    let mut steady_state = true;
    for y in 0..input.len() {
        let row = input.get_mut(y).expect("I need a row");
        for x in 0..row.len() {
            let current_char = row.get_mut(x).expect("need a char");
            if current_char == &'#' && heatmap[y][x] > max_heatmap_count {
                *current_char = 'L';
                steady_state = false;
            } else if current_char == &'L' && heatmap[y][x] == 0 {
                *current_char = '#';
                steady_state = false;
            }
        }
    }
    steady_state
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut puzzle = parser.map;

    let mut count = 0;
    let mut steady_state = false;
    while !steady_state {
        count += 1;
        steady_state = tick2(&mut puzzle);
    }
    println!("{} with {} ticks", puzzle.iter()
    .map(|x : &Vec<char>| x.iter().filter(|y| **y == '#').count())
    .sum::<usize>(), count);
}



fn tick2(input : &mut Vec<Vec<char>>) -> bool {
    let mut heatmap : Vec<Vec<i32>> = vec![vec![0; input.get(0).expect("one should be here").len()]; input.len()];
    let mut y = 0;
    for row in input.iter() {
        let mut x = 0;
        for c in row {
            if c == &'#' {
                mark_line_of_sight(&mut heatmap, input, x, y)
            }
            x += 1;
        }
        y += 1;
    }
    
    update_using_heatmap(input, &heatmap, 4)
}

fn mark_line_of_sight(heatmap : &mut Vec<Vec<i32>>, input : &Vec<Vec<char>>, point_x: usize, point_y: usize) {
    let mut directions : Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        //(0, 0), //no need to self-check!
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        ];
    for direction in &mut directions {
        let mut y = point_y;
        let mut x = point_x;
        loop {
            let maybe = checked_add(y, direction.0);
            if maybe.is_none() {
                break;
            }
            y = maybe.expect("y expected");

            let maybe = checked_add(x, direction.1);
            if maybe.is_none() {
                break;
            }
            x = maybe.expect("x expected");
            if let Some(val) = heatmap.get_mut(y).and_then(|row| row.get_mut(x)) {
                *val += 1;
            } else {
                break;
            }
            if input[y][x] != '.' {
                break;
            }
        }
    }
}

fn checked_add(x : usize, i : i32) -> Option<usize> {
    if i == 0 {
        return Some(x);
    }
    if i > 0 {
        return x.checked_add(i as usize);
    }
    x.checked_sub((i * -1) as usize)
}



struct Parser {
    map : Vec<Vec<char>>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            map: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        self.map.push(line.chars().collect());
    }
}
