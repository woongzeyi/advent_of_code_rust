use std::fs;
use itertools::Itertools;

const TARGET_SUM: usize = 2020;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|content| content.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    println!("Two entries: {}", product_of_n_entries_sum_to_target(&contents, 2, &TARGET_SUM).unwrap());
    println!("Three entries: {}", product_of_n_entries_sum_to_target(&contents, 3, &TARGET_SUM).unwrap());
}

fn product_of_n_entries_sum_to_target(input: &Vec<usize>, n: usize, target: &usize) -> Option<usize> {
    for combination in input.iter().cloned().combinations(n) {
        if &combination.iter().sum::<usize>() == target {
            return Some(combination.iter().product());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: [usize; 6] = [1721, 979, 366, 299, 675, 1456];
    const TEST_TARGET: usize = 2020;

    #[test]
    fn product_of_two_entries_calculation() {
        assert_eq!(
            product_of_n_entries_sum_to_target(&TEST_INPUT.to_vec(), 2, &TEST_TARGET),
            Some(514579)
        )
    }

    #[test]
    fn product_of_three_entries_calculation() {
        assert_eq!(
            product_of_n_entries_sum_to_target(&TEST_INPUT.to_vec(), 3, &TEST_TARGET),
            Some(241861950)
        )
    }
}