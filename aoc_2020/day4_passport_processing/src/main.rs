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
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn validate_passport_fn_test() {
        assert!(validate_passport(
            &[
                ("ecl", "gry"),
                ("pid", "860033327"),
                ("eyr", "2020"),
                ("hcl", "#fffffd"),
                ("byr", "1937"),
                ("iyr", "2017"),
                ("cid", "147"),
                ("hgt", "183cm"),
            ]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<String, String>>(),
            &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        ));
        assert!(!validate_passport(
            &[
                ("ecl", "gry"),
                ("pid", "860033327"),
                ("hcl", "#fffffd"),
                ("byr", "1937"),
                ("iyr", "2017"),
                ("cid", "147"),
                ("hgt", "183cm"),
            ]
            .iter()
            .map(|(key, value)| (key.to_string(), value.to_string()))
            .collect::<HashMap<String, String>>(),
            &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        ));
    }

    #[test]
    fn validate_value_fn_test() {
        todo!();
    }
}
