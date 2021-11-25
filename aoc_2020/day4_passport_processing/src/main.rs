use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .split("\r\n\r\n")
        .map(|f| {
            let mut hashmap = HashMap::new();
            f.split_whitespace().for_each(|d| {
                let s = d.split(':').collect::<Vec<&str>>();
                let _ = &hashmap.insert(s[0].to_string(), s[1].to_string());
            });
            hashmap
        })
        .collect::<Vec<HashMap<String, String>>>();
    println!(
        "Part 1: {}",
        input.iter().fold(0, |acc, f| {
            acc + validate_passport(f, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]) as usize
        })
    );
    println!(
        "Part 2: {}",
        input.iter().fold(0, |acc, f| {
            acc + (validate_passport(f, &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"])
                && validate_value(f)) as usize
        })
    );
}

fn validate_passport(passport: &HashMap<String, String>, required_fields: &[&str]) -> bool {
    for field in required_fields {
        if passport.get(*field).is_none() {
            return false;
        }
    }
    true
}

fn validate_value(passport: &HashMap<String, String>) -> bool {
    for (key, value) in passport {
        match key.as_str() {
            "byr" => match value.parse::<usize>() {
                Ok(value) => {
                    if !(1920..=2002).contains(&value) {
                        return false;
                    }
                }
                Err(_) => return false,
            },
            "iyr" => match value.parse::<usize>() {
                Ok(value) => {
                    if !(2010..=2020).contains(&value) {
                        return false;
                    }
                }
                Err(_) => return false,
            },
            "eyr" => match value.parse::<usize>() {
                Ok(value) => {
                    if !(2020..=2030).contains(&value) {
                        return false;
                    }
                }
                Err(_) => return false,
            },
            "hgt" => match value {
                cm if cm.ends_with("cm") => match cm.get(..cm.find("cm").unwrap()) {
                    Some(value) => {
                        let parsed = value.parse::<usize>();
                        if parsed.is_err() {
                            return false;
                        }
                        let parsed = parsed.unwrap();
                        if !(150..=193).contains(&parsed) {
                            return false;
                        }
                    }
                    None => return false,
                },
                inch if inch.ends_with("in") => match inch.get(..inch.find("in").unwrap()) {
                    Some(value) => {
                        let parsed = value.parse::<usize>();
                        if parsed.is_err() {
                            return false;
                        }
                        let parsed = parsed.unwrap();
                        if !(59..=76).contains(&parsed) {
                            return false;
                        }
                    }
                    None => return false,
                },
                _ => return false,
            },
            "hcl" => {
                if value.starts_with('#') {
                    match value.get(1..) {
                        Some(value) => {
                            if !value
                                .chars()
                                .all(|f| ('0'..='9').contains(&f) && ('a'..='f').contains(&f))
                            {
                                return false;
                            }
                        }
                        None => return false,
                    }
                } else {
                    return false;
                }
            }
            "ecl" => {
                if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()) {
                    return false;
                }
            }
            "pid" => {
                if !(value.len() == 9 && value.chars().all(|f| f.is_numeric())) {
                    return false;
                }
            }
            "cid" => { /* Ignored */ }
            _ => panic!("Unrecognized field"),
        }
    }
    true
}
