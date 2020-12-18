fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(18, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}


pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut result = 0;
    for expr in parser.input {
        let tmp = evaluate(&expr);
        result += tmp;
    }
    println!("{} is the total result", result);
}

fn evaluate(expr : &str) -> i64 {
    let chars : Vec<char> = expr.chars().collect();
    evaluate_internal(&chars, &mut 0)
}

fn evaluate_internal(expr : &Vec<char>, index : &mut usize) -> i64 {
    let mut result = 0;
    let mut number_to_use : i64 = 0;
    let mut operator = ' ';

    while *index < expr.len() {
        match expr[*index] {
            '(' => {
                *index += 1;
                number_to_use = evaluate_internal(expr, index);
            },
            ')' => {
                *index += 1;
                return commit_operator(result, number_to_use, operator);
            },
            '+' | '*' => {
                result = commit_operator(result, number_to_use, operator);
                operator = expr[*index];
                number_to_use = 0;
                *index += 1;
            }
            _ => {
                number_to_use *= 10;
                let number_to_add : i64 = expr[*index].to_string().parse().expect("should be a number");
                number_to_use += number_to_add;
                *index += 1;
            }
        }
    }
    commit_operator(result, number_to_use, operator)
}

fn commit_operator(nr1: i64, nr2: i64, operator : char) -> i64 {
    let result;
    match operator {
        '+' => result = nr1 + nr2,
        '*' => result = nr1 * nr2,
        ' ' => result = nr2,
        _ => panic!("unknown operator"),
    }
    result
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let mut result = 0;
    for expr in parser.input {
        let tmp = evaluate2(&expr);
        result += tmp;
    }
    println!("{} is the total result", result);
}

fn evaluate2(expr : &str) -> i64 {
    let chars : Vec<char> = expr.chars().collect();
    evaluate2_internal(&chars, &mut 0)
}

fn evaluate2_internal(expr : &Vec<char>, index : &mut usize) -> i64 {
    let mut result = 0;
    let mut numbers_to_multiply : Vec<i64> = vec![];
    let mut number_to_use : i64 = 0;
    let mut operator = ' ';

    while *index < expr.len() {
        match expr[*index] {
            '(' => {
                *index += 1;
                number_to_use = evaluate2_internal(expr, index);
            },
            ')' => {
                *index += 1;
                return final_commit(result, number_to_use, operator, &mut numbers_to_multiply);
            },
            '+' | '*' => {
                result = commit_operator2(result, number_to_use, operator, &mut numbers_to_multiply);
                operator = expr[*index];
                number_to_use = 0;
                *index += 1;
            }
            _ => {
                number_to_use *= 10;
                let number_to_add : i64 = expr[*index].to_string().parse().expect("should be a number");
                number_to_use += number_to_add;
                *index += 1;
            }
        }
    }
    final_commit(result, number_to_use, operator, &mut numbers_to_multiply)
}

fn final_commit(nr1: i64, nr2: i64, operator : char, numbers_to_multiply : &mut Vec<i64>) -> i64 {
    let mut result = commit_operator2(nr1, nr2, operator, numbers_to_multiply);
    for x in numbers_to_multiply {
        result *= *x;
    }
    result
}

fn commit_operator2(nr1: i64, nr2: i64, operator : char, numbers_to_multiply : &mut Vec<i64>) -> i64 {
    let result;
    match operator {
        '+' => result = nr1 + nr2,
        '*' => {
            numbers_to_multiply.push(nr1);
            result = nr2;
        },
        ' ' => result = nr2,
        _ => panic!("unknown operator"),
    }
    result
}

struct Parser {
    input: Vec<String>,
}
impl Parser {
    fn new() -> Parser {
        Parser {
            input: vec![],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }

        self.input.push(line.replace(" ", ""));
    }
}