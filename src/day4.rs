use std::collections::HashMap;

fn parse_input(sample_data: bool) -> Parser {
    let mut parser = Parser::new();
    crate::filehelper::read_file(4, sample_data, &mut |s: String| parser.parse_string(s));
    parser.flush();
    parser
}

pub fn exec1(sample_data: bool) {
    let parser = parse_input(sample_data);

    let valid_properties: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        //"cid".to_string(), //this will be our secret
    ];

    let mut passports_found = 0;

    for i in 0..parser.get_result_size() {
        let passport = parser.get_result(i);
        let mut valid = true;
        for j in 0..valid_properties.len() {
            if !passport.has_property(valid_properties.get(j).expect("language noped out")) {
                valid = false;
                break;
            }
        }
        if valid {
            passports_found += 1;
        }
    }
    println!("{} valid passports found", passports_found)
}

pub fn exec2(sample_data: bool) {
    let parser = parse_input(sample_data);

    let valid_properties: Vec<String> = vec![
        "byr".to_string(),
        "iyr".to_string(),
        "eyr".to_string(),
        "hgt".to_string(),
        "hcl".to_string(),
        "ecl".to_string(),
        "pid".to_string(),
        //"cid".to_string(), //sst ;) 
    ];

    let mut passports_found = 0;

    for i in 0..parser.get_result_size() {
        let passport = parser.get_result(i);
        let mut valid = true;
        for j in 0..valid_properties.len() {
            let property = valid_properties.get(j).expect("language noped out");
            if !passport.has_property(property) || !passport.validate_property(property) {
                valid = false;
                break;
            }
        }
        if valid {
            passports_found += 1;
        }
    }
    println!("{} valid passports found", passports_found)
}

struct Passport {
    properties: HashMap<String, String>,
}
impl Passport {
    fn new() -> Passport {
        Passport {
            properties: HashMap::new(),
        }
    }

    fn add_property(&mut self, key: String, value: String) {
        self.properties.insert(key, value);
    }

    fn has_property(&self, property: &String) -> bool {
        self.properties.contains_key(property)
    }

    fn validate_property(&self, property: &String) -> bool {
        match property.as_str() {
            "byr" => self.check_byr(),
            "iyr" => self.check_iyr(),
            "eyr" => self.check_eyr(),
            "hgt" => self.check_hgt(),
            "hcl" => self.check_hcl(),
            "ecl" => self.check_ecl(),
            "pid" => self.check_pid(),
            _ => false,
        }
    }

    fn check_between(property : &String, left_included_bound : i32, right_included_bound : i32) -> bool {
        let optional_value : Result<i32, _> = property.parse();
        if optional_value.is_err() {
            return false;
        }
        let value = optional_value.expect("language told me it was good!");
        value >= left_included_bound && value <= right_included_bound
    }

    fn check_byr(&self) -> bool {
        Passport::check_between( self.properties.get("byr").expect("should be here"), 1920, 2002)
    }

    fn check_iyr(&self) -> bool {
        Passport::check_between( self.properties.get("iyr").expect("should be here"), 2010, 2020)
    }

    fn check_eyr(&self) -> bool {
        Passport::check_between( self.properties.get("eyr").expect("should be here"), 2020, 2030)
    }

    fn check_hgt(&self) -> bool {
        let hgt = self.properties.get("hgt").expect("should be here");
        if hgt.len() < 4 {
            return false;
        }
        let unit : String = hgt.chars().skip(hgt.len() - 2).collect();
        let nbr_str : String = hgt.chars().take(hgt.len() - 2).collect();
        let optional_value : Result<i32, _> = nbr_str.parse();
        if optional_value.is_err() {
            return false;
        }
        let value = optional_value.expect("language told me it was good!");
        match unit.as_str() {
            "cm" => value >= 150 && value <= 193,
            "in" => value >= 59 && value <= 76,
            _ => false,
        }
    }

    fn check_hcl(&self) -> bool {
        let hcl = self.properties.get("hcl").expect("should be here");
        if hcl.len() != 7 {
            return false;
        }
        if hcl.chars().next().expect("should be here") != '#' {
            return false;
        }
        hcl.chars().skip(1).all(|c| c.is_ascii_hexdigit())
    }

    fn check_ecl(&self) -> bool {
        let ecl = self.properties.get("ecl").expect("should be here");
        match ecl.as_str() {
            "amb" => true,
            "blu" => true,
            "brn" => true,
            "gry" => true,
            "grn" => true,
            "hzl" => true,
            "oth" => true,
            _ => false,
        }
    }

    fn check_pid(&self) -> bool {
        let pid = self.properties.get("pid").expect("should be here");
        if pid.len() != 9 {
            return false;
        }
        pid.chars().all(|c| c.is_numeric())
    }
}

struct Parser {
    passports: Vec<Passport>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            passports: vec![Passport::new()],
        }
    }

    pub fn flush(&mut self) {
        self.passports.push(Passport::new());
    }

    pub fn parse_string(&mut self, line: String) {
        if line == "" {
            self.flush();
            return;
        }
        let passport_idx = self.passports.len() - 1;
        let current_passport = self
            .passports
            .get_mut(passport_idx)
            .expect("what are you even doing? this should not happen!");
        let iter = line.split_whitespace();
        for property_string in iter {
            let mut key_value = property_string.split(":");
            current_passport.add_property(
                key_value
                    .next()
                    .expect("the key should be filled out")
                    .to_string(),
                key_value
                    .next()
                    .expect("the value should be filled out")
                    .to_string(),
            )
        }
    }

    fn get_result(&self, index: usize) -> &Passport {
        self.passports
            .get(index)
            .expect("No password - BIG problem!")
    }

    fn get_result_size(&self) -> usize {
        self.passports.len()
    }
}
