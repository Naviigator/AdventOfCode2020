pub fn exec1(sample_data: bool) {
    let mut parser = Parser::new();
    crate::filehelper::read_file(1, sample_data, &mut |s: String| parser.parse_string(s));
    
    for i in 0..parser.get_result_size() {
        for j in (i + 1)..parser.get_result_size() {
            let addition = parser.get_result(i) + parser.get_result(j);
            if addition == 2020 {
                println!("{}", parser.get_result(i) * parser.get_result(j));
                return;
            }
        }
    }
    println!("what happened?");
}

pub fn exec2(sample_data: bool) {
    let mut parser = Parser::new();
    crate::filehelper::read_file(1, sample_data, &mut |s: String| parser.parse_string(s));

    for i in 0..parser.get_result_size() {
        for j in (i + 1)..parser.get_result_size() {
            for k in (j + 1)..parser.get_result_size() {
                let addition = parser.get_result(i) + parser.get_result(j) + parser.get_result(k);
                if addition == 2020 {
                    println!(
                        "{}",
                        parser.get_result(i) * parser.get_result(j) * parser.get_result(k)
                    );
                    return;
                }
            }
        }
    }
    println!("what happened?");
}

struct Parser {
    results: Vec<i64>,
}
impl Parser {
    fn new() -> Parser {
        Parser { results: vec![] }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        self.results.push(line.parse().unwrap());
    }

    fn get_result(&self, index: usize) -> i64 {
        self.results[index]
    }

    fn get_result_size(&self) -> usize {
        self.results.len()
    }
}
