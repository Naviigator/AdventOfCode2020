use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(7, sample_data, &mut |s: String| parser.parse_string(s));
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut found_cache : HashMap<String, bool> = HashMap::new();
    let to_find = "shiny gold";
    for current_bag in parser.bags.keys() {
        if current_bag == to_find {
            continue;
        }
        fill_cache(to_find, current_bag, &parser.bags, &mut found_cache);
    }

    println!("{} bags can contain {}", found_cache.values().filter(|x| **x).count(), to_find);
}

fn fill_cache(bag_to_find : &str, bag_name : &str, bags : &HashMap<String, Vec<BagContent>>, cache : &mut HashMap<String, bool>) {
    if !cache.contains_key(bag_name) {
        let mut found = false;
        for sub_bag in bags.get(bag_name).expect("bag should exist") {
            let bag_type = &sub_bag.bag_type;
            if bag_type == bag_to_find {
                found = true;
                break;
            } else if !cache.contains_key(bag_type) {
                fill_cache(bag_to_find, bag_type, bags, cache);
            }
            if *cache.get(bag_type).expect("cache not filled") {
                found = true;
                break;
            }
        }
        cache.insert(bag_name.to_string(), found);
    }
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);
    let mut cache : HashMap<String, i32> = HashMap::new();
    let to_find = "shiny gold";
    fill_count_cache(to_find,  &parser.bags, &mut cache);

    println!("{} bags can contain {}", cache.get(to_find).expect("didn't find a result, weird!"), to_find);
}

fn fill_count_cache(bag_name : &str, bags : &HashMap<String, Vec<BagContent>>, cache : &mut HashMap<String, i32>) {
    if !cache.contains_key(bag_name) {
        let mut count = 0;
        for sub_bag in bags.get(bag_name).expect("bag should exist") {
            let bag_type = &sub_bag.bag_type;
            if !cache.contains_key(bag_type) {
                fill_count_cache(bag_type, bags, cache);
            }
            count += sub_bag.amount * (cache.get(bag_type).expect("cache not filled") + 1);
        }
        cache.insert(bag_name.to_string(), count);
    }
}

struct BagContent {
    bag_type: String,
    amount: i32,
}
impl BagContent {
    fn new(bag_type: String, amount: i32) -> BagContent {
        BagContent {
            bag_type: bag_type,
            amount: amount,
        }
    }
}

struct Parser {
    bags : HashMap<String, Vec<BagContent>>
}
impl Parser {
    fn new() -> Parser {
        Parser {
            bags: HashMap::new(),
        }
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            return;
        }
        let subject_content : Vec<&str> = line.split(" bags contain ").collect();
        let subject = subject_content.get(0).expect("subject expected");
        let all_content = subject_content.get(1).expect("we need more content!");

        let mut bag_content : Vec<BagContent> = vec![];
        if all_content != &"no other bags." {
            for content in all_content.split(", ") {
                let first_space = content.find(" ").expect("we need a number");
                let count = content[0..first_space].parse().expect("this is NaN");
                let bag_loc = content.find(" bag").expect("we need a bag string");
                bag_content.push(BagContent::new(content[first_space + 1..bag_loc].to_string(), count));
            }
        }
        self.bags.insert(subject.to_string(), bag_content);
    }
}
