fn parse_input(sample_data : bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(2, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut valid_count = 0;

    for i in 0..parser.get_result_size() {
        let specs = parser.get_result(i);
        let char_count = specs.password.matches(specs.character).count();
        if char_count >= specs.min_count && char_count <= specs.max_count {
            valid_count += 1;
        }
    }
    println!("found {} matching passwords", valid_count);
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut valid_count = 0;

    for i in 0..parser.get_result_size() {
        let specs = parser.get_result(i);
        let password_vec : Vec<char> = specs.password.chars().collect();
        let first = *password_vec.get(specs.min_count - 1).unwrap_or(&' ');
        let second = *password_vec.get(specs.max_count - 1).unwrap_or(&' ');
        
        if (first == specs.character || second == specs.character) && first != second {
            valid_count += 1;
        }
    }
    println!("found {} matching passwords", valid_count);
}

struct PasswordSpecs {
    min_count: usize,
    max_count: usize, 
    character: char,
    password: String,
}

struct Parser {
    results: Vec<PasswordSpecs>,
}
impl Parser {
    fn new() -> Parser {
        Parser { results: vec![] }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let mut iter = line.split_whitespace();
        let min_max = iter.next().expect("wrong input");
        let character_string = iter.next().expect("wrong input");
        let password = iter.next().expect("wrong input");

        let mut min_max_iter = min_max.split("-");
        let min: usize = min_max_iter
            .next()
            .expect("wrong input")
            .parse()
            .expect("wrong input");
        let max: usize = min_max_iter
            .next()
            .expect("wrong input")
            .parse()
            .expect("wrong input");

        self.results.push(PasswordSpecs {
            min_count: min,
            max_count: max,
            character: character_string.chars().next().expect("char expected"),
            password: password.to_string(),
        });
    }

    fn get_result(&self, index: usize) -> &PasswordSpecs {
        &self.results[index]
    }

    fn get_result_size(&self) -> usize {
        self.results.len()
    }
}
