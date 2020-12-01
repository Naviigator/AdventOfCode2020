pub fn exec1(sample_data : bool) {
    let mut parser = Parser::new();
    crate::filehelper::read_file(0, sample_data, &mut |s: String|{parser.parse_string(s)});
    println!("{} {}!", parser.get_result(0), parser.get_result(1));
}

pub fn exec2(_sample_data : bool) {
    println!("part 2 dummy");
}

struct Parser {
    results: Vec<String>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            results: vec![],
        }
    }

    pub fn parse_string(&mut self, line : String) {
        self.results.push(line);
    }

    fn get_result(&self, index: usize) -> String {
        self.results[index].clone()
    }
}