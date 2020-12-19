use std::collections::HashMap;
use std::collections::HashSet;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(19, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut result = 0;
    
    for input in &parser.input {
        if parser.is_valid(&input) {
            result += 1;
        } else {
        }
    }
    println!("{} inputs match rule 0", result);
}

pub fn exec2(sample_data: bool) {
    let mut parser = parse_input(sample_data);
    parser.initialise2();
    let mut result = 0;

    
    for input in &parser.input {
        if parser.is_valid(&input) {
            result += 1;
        }
    }
    println!("{} inputs match rule 0", result);
}

struct LinksOrChar {
    links : Vec<Vec<usize>>,
    character : char,
} impl LinksOrChar {
    fn new_vec(links : Vec<Vec<usize>>) -> LinksOrChar {
        LinksOrChar{links: links, character: ' '}
    }
    fn new_char(character : char) -> LinksOrChar {
        LinksOrChar{links: vec![], character: character}
    }

    fn new8() -> LinksOrChar {
        let mut links : Vec<Vec<usize>> = vec![];
        let mut first = vec![];
        first.push(42);
        links.push(first);
        let mut second = vec![];
        second.push(42);
        second.push(8);
        links.push(second);
        LinksOrChar::new_vec(links)
    }

    fn new11() -> LinksOrChar {
        let mut links = vec![];
        let mut first = vec![];
        first.push(42);
        first.push(31);
        links.push(first);
        let mut second = vec![];
        second.push(42);
        second.push(11);
        second.push(31);
        links.push(second);
        LinksOrChar::new_vec(links)
    }
}

struct Parser {
    raw: HashMap<usize, LinksOrChar>,
    input: Vec<String>,
    reading_rules: bool,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            raw: HashMap::new(),
            input: vec![],
            reading_rules: true,
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            self.reading_rules = false;
            return;
        }

        if !self.reading_rules {
            self.input.push(line);
            return;
        }
        let values : Vec<&str> = line.split(": ").collect();
        let id = values[0].parse().expect("id should be a number");
        let rules_str = values[1];
        if rules_str.starts_with("\"") {
            let character = rules_str.chars().nth(1).expect("should be a char");
            self.raw.insert(id, LinksOrChar::new_char(character));
            return;
        }
        let mut all_links = vec![];
        let mut links : Vec<usize> = vec![];
        for token in rules_str.split(" ") {
            if token == "|" {
                all_links.push(links);
                links = vec![];
            } else {
                links.push(token.parse().expect("should be a number"));
            }
        }
        all_links.push(links);
        self.raw.insert(id, LinksOrChar::new_vec(all_links));
    }

    pub fn initialise2(&mut self) {
        self.raw.insert(8, LinksOrChar::new8());
        self.raw.insert(11,  LinksOrChar::new11());
    }

    pub fn is_valid(&self, input : &str) -> bool {
        let sizes = self.get_valid_sizes(input, 0);
        let mut result = false;
        for size in sizes {
            if size == input.len() {
                result = true;
                break;
            }
        }
        result
    }

    fn get_valid_sizes(&self, input : &str, rule: usize) -> HashSet<usize> {
        if input.is_empty() {
            return HashSet::new();
        }
        let me = self.raw.get(&rule).expect("should exist");
        if me.character != ' ' {
            let mut result = HashSet::new();
            if input.chars().nth(0).expect("we need a character") == me.character {
                result.insert(1);
            }
            return result;
        }
        
        let mut valid_sizes = HashSet::new();
        for links in &me.links {
            let mut current_valid_sizes = HashSet::new();
            for i in 0..links.len() {
                if i == 0 {
                    current_valid_sizes = self.get_valid_sizes(input, links[i]);
                    if current_valid_sizes.is_empty() {
                        break;
                    }
                } else {
                    let mut new_valid_sizes = HashSet::new();
                    for valid_size in current_valid_sizes {
                        let result = self.get_valid_sizes(&input[valid_size..], links[i]);
                        for result_size in result {
                            new_valid_sizes.insert(result_size + valid_size);
                        }
                    }
                    current_valid_sizes = new_valid_sizes;
                }
            }
            valid_sizes.extend(current_valid_sizes);
        }
        valid_sizes
    }
}