use itertools::Itertools;

#[derive(Debug)]
struct InputLine {
    policy: Policy,
    password: String,
}

#[derive(Debug)]
struct Policy {
    range: std::ops::Range<usize>,
    char: char,
}

fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|f| {
            let splitted = f.split(": ").collect::<Vec<&str>>();
            let policy = splitted[0].split(' ').collect::<Vec<&str>>();
            let policy_range = policy[0]
                .split('-')
                .map(|f| f.parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            let policy_char = policy[1].chars().next().unwrap();
            let password = splitted[1].to_string();

            InputLine {
                policy: Policy {
                    range: policy_range[0]..policy_range[1],
                    char: policy_char,
                },
                password,
            }
        })
        .collect::<Vec<InputLine>>();

    println!(
        "Part 1: {}",
        input
            .iter()
            .fold(0, |acc, f| match contains_valid_char_count(f) {
                true => acc + 1,
                false => acc,
            })
    );
    println!(
        "Part 2: {}",
        input
            .iter()
            .fold(0, |acc, f| match exactly_one_char_at_either_pos(f) {
                true => acc + 1,
                false => acc,
            })
    );
}

fn contains_valid_char_count(policy_and_password: &InputLine) -> bool {
    let char_count = match policy_and_password
        .password
        .chars()
        .counts()
        .get(&policy_and_password.policy.char)
    {
        Some(count) => *count,
        None => return false,
    };
    let std::ops::Range { start, end } = policy_and_password.policy.range;
    start <= char_count && char_count <= end
}

fn exactly_one_char_at_either_pos(policy_and_password: &InputLine) -> bool {
    let std::ops::Range {
        start: one,
        end: two,
    } = policy_and_password.policy.range;
    let policy_char = policy_and_password.policy.char;
    let chars = policy_and_password.password.chars();
    let pos_one = match chars.clone().nth(one - 1) {
        Some(char) => char,
        None => return false,
    };
    let pos_two = match chars.clone().nth(two - 1) {
        Some(char) => char,
        None => return false,
    };
    (pos_one == policy_char) ^ (pos_two == policy_char)
}

// cSpell:disable
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contains_valid_char_count_fn_test() {
        assert!(contains_valid_char_count(&InputLine {
            policy: Policy {
                range: 1..2,
                char: 'a'
            },
            password: "abbc".to_string()
        }));
        assert!(!contains_valid_char_count(&InputLine {
            policy: Policy {
                range: 1..2,
                char: 'a'
            },
            password: "bbcd".to_string()
        }));
    }

    #[test]
    fn exactly_one_char_at_either_pos_fn_test() {
        assert!(exactly_one_char_at_either_pos(&InputLine {
            policy: Policy {
                range: 1..2,
                char: 'a'
            },
            password: "abbc".to_string()
        }));
        assert!(!exactly_one_char_at_either_pos(&InputLine {
            policy: Policy {
                range: 1..2,
                char: 'a'
            },
            password: "aabc".to_string()
        }));
        assert!(!exactly_one_char_at_either_pos(&InputLine {
            policy: Policy {
                range: 1..2,
                char: 'a'
            },
            password: "bbcd".to_string()
        }));
    }
}
// cSpell:enable