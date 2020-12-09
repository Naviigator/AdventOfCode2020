fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(9, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut cache_size = 25;
    if sample_data {
        cache_size = 5;
    }
    
    let mut cache = parser.instructions.iter().take(cache_size).map(|x| *x).collect();
    for i in cache_size..parser.instructions.len() {
        let current_val = *parser.instructions.get(i).expect("expected a number here");
        if !is_valid(current_val, &cache) {
            println!("the first invalid number is {}", current_val);
            break;
        }
        cache[i % cache_size] = current_val;
    }

}

fn is_valid(current_val : i64, cache : &Vec<i64>) -> bool {
    let cache_size = cache.len();
    for j in 0..(cache_size - 1) {
        for k in ((j + 1)..cache_size).rev() {
            let first = cache.get(j).expect("j expected");
            let second = cache.get(k).expect("k expected");

            if first + second == current_val {
                return true;
            }
        }
    }
    false
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut cache_size = 25;
    if sample_data {
        cache_size = 5;
    }
    
    let mut cache = parser.instructions.iter().take(cache_size).map(|x| *x).collect();
    for i in cache_size..parser.instructions.len() {
        let current_val = *parser.instructions.get(i).expect("expected a number here");
        if !is_valid(current_val, &cache) {
            for j in 0..i {
                let mut count = 0;
                for k in j..i {
                    count += parser.instructions.get(k).expect("this seems to be a special K");
                    if count == current_val {
                        let range : Vec<&i64> = parser.instructions.iter().skip(j).take(k - j).collect();
                        println!("The result is: {}", **range.iter().min().expect("come on now") + **range.iter().max().expect("seriously?"));
                        return;
                    }
                    if count > current_val {
                        break;
                    }
                }
            }
            break;
        }
        cache[i % cache_size] = current_val;
    }
}

struct Parser {
    instructions : Vec<i64>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            instructions: Vec::new(),
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }

        self.instructions.push(line.parse().expect("should be a number"));
    }
}
