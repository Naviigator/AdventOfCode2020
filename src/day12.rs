fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(12, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

const NORTH : (i32, i32) =  (0, 1);
const EAST : (i32, i32) =  (1, 0);
const SOUTH : (i32, i32) =  (0, -1);
const WEST : (i32, i32) =  (-1, 0);

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut looking_direction = EAST;

    let mut current_location = (0, 0);

    for current_route in parser.route {
        let mut amount = current_route.amount;
        let mut moving_direction = (0, 0);
        match current_route.direction.as_str() {
            "N" => moving_direction = NORTH,
            "E" => moving_direction = EAST,
            "S" => moving_direction = SOUTH,
            "W" => moving_direction = WEST,
            "F" => moving_direction = looking_direction,
            "L" => {
                while amount > 0 {
                    amount -= 90;
                    looking_direction = left(looking_direction);
                }
                while amount < 0 {
                    amount += 90;
                    looking_direction = right(looking_direction);
                }
            },
            "R" => {
                while amount > 0 {
                    amount -= 90;
                    looking_direction = right(looking_direction);
                }
                while amount < 0 {
                    amount += 90;
                    looking_direction = left(looking_direction);
                }
            },
            _ => (),
        };
        current_location.0 += moving_direction.0 * amount;
        current_location.1 += moving_direction.1 * amount;
    }
    println!("ended up in x: {}, y: {}, manhattan distance of {}", current_location.0, current_location.1, current_location.0.abs() + current_location.1.abs());
}

fn left(direction : (i32, i32)) -> (i32, i32) {
    (direction.1 * -1, direction.0)
}

fn right(direction : (i32, i32)) -> (i32, i32) {
    (direction.1, direction.0 * -1)
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut current_location = (0, 0);
    let mut waypoint = (10, 1);

    for current_route in parser.route {
        let mut amount = current_route.amount;
        let mut moving_direction = (0, 0);
        let mut what_to_move = &mut waypoint;
        match current_route.direction.as_str() {
            "N" => moving_direction = NORTH,
            "E" => moving_direction = EAST,
            "S" => moving_direction = SOUTH,
            "W" => moving_direction = WEST,
            "F" => {
                moving_direction = waypoint;
                what_to_move = &mut current_location;
            },
            "L" => {
                while amount < 0 {
                    amount += 360;
                }
                while amount > 0 {
                    amount -= 90;
                    let result = left(*what_to_move);
                    what_to_move.0 = result.0;
                    what_to_move.1 = result.1;
                }
            },
            "R" => {
                while amount < 0 {
                    amount += 360;
                }
                while amount > 0 {
                    amount -= 90;
                    let result = right(*what_to_move);
                    what_to_move.0 = result.0;
                    what_to_move.1 = result.1;
                }
            },
            _ => (),
        };
        what_to_move.0 += moving_direction.0 * amount;
        what_to_move.1 += moving_direction.1 * amount;
    }
    println!("ended up in x: {}, y: {}, manhattan distance of {}", current_location.0, current_location.1, current_location.0.abs() + current_location.1.abs());
}


struct RouteDescription {
    direction: String,
    amount: i32,
}
impl RouteDescription {
    fn new(direction: String, amount: i32) -> RouteDescription {
        RouteDescription {
            direction: direction,
            amount: amount,
        }
    }
}

struct Parser {
    route : Vec<RouteDescription>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            route: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        self.route.push(RouteDescription::new(line[0..1].to_string(), line[1..].parse().expect("need a number")));
    }
}