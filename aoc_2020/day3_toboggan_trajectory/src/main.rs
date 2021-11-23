fn main() {
    let input = std::fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|f| f.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();
    println!("Part 1: {}", tree_encounter_count(3, 1, &input));
    println!(
        "Part 1: {}", 
        tree_encounter_count(1, 1, &input) * 
        tree_encounter_count(3, 1, &input) * 
        tree_encounter_count(5, 1, &input) * 
        tree_encounter_count(7, 1, &input) * 
        tree_encounter_count(1, 2, &input)
    );
}

fn tree_encounter_count(slope_right: usize, slope_down: usize, map: &Vec<Vec<char>>) -> usize {
    map.into_iter()
        .step_by(slope_down)
        .enumerate()
        .skip(1)
        .fold(0, |acc, (idx, row)| {
            let pos_to_access = idx * slope_right % row.len();
            match row.into_iter().nth(pos_to_access) {
                Some(char) => {
                    if *char == '#' {
                        acc + 1
                    } else {
                        acc
                    }
                }
                None => panic!("Current column index: {}, Position to access: {}", idx, pos_to_access),
            }
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_encounter_count_fn_test() {
        assert_eq!(
            7,
            tree_encounter_count(
                3,
                1,
                &"..##.........##.........##.........##.........##.........##....... \n\
                #...#...#..#...#...#..#...#...#..#...#...#..#...#...#..#...#...#.. \n\
                .#....#..#..#....#..#..#....#..#..#....#..#..#....#..#..#....#..#. \n\
                ..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.#..#.#...#.# \n\
                .#...##..#..#...##..#..#...##..#..#...##..#..#...##..#..#...##..#. \n\
                ..#.##.......#.##.......#.##.......#.##.......#.##.......#.##..... \n\
                .#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....#.#.#.#....# \n\
                .#........#.#........#.#........#.#........#.#........#.#........# \n\
                #.##...#...#.##...#...#.##...#...#.##...#...#.##...#...#.##...#... \n\
                #...##....##...##....##...##....##...##....##...##....##...##....# \n\
                .#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#.#..#...#.#"
                    .lines()
                    .map(|f| f.chars().collect::<Vec<char>>())
                    .collect::<Vec<Vec<char>>>()
            )
        );
    }
}
