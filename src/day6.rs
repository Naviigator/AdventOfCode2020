use std::collections::HashSet;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(6, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let result : usize = parser.groups.iter().map(|group| group.count()).sum();
    println!("{} questions were answered 'YES' to", result);
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let result : usize = parser.groups.iter().map(|group| group.count_all()).sum();
    println!("{} questions were answered 'YES' to", result);
}

struct Group {
    answers: HashSet<char>,
    all_answers : HashSet<char>,
    fresh_group : bool,
}
impl Group {
    fn new() -> Group {
        Group {
            answers: HashSet::new(),
            all_answers: HashSet::new(),
            fresh_group: true,
        }
    }

    fn add (&mut self, input : String) {
        let temp : HashSet<char> = input.chars().collect();
        self.answers.extend(temp.iter());

        if self.fresh_group {
            self.all_answers = temp;
            self.fresh_group = false;
        } else {
            self.all_answers = self.all_answers.intersection(&temp).map(|x| *x).collect();
        }
    }

    fn count(&self) -> usize {
        self.answers.len()
    }

    fn count_all(&self) -> usize {
        self.all_answers.len()
    }
}

struct Parser {
    groups : Vec<Group>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            groups: vec![Group::new()],
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            self.groups.push(Group::new());
            return;
        }
        let group_idx = self.groups.len() - 1;
        let current_group = self.groups.get_mut(group_idx).expect("where has the group gone?");
        current_group.add(line);
    }
}
