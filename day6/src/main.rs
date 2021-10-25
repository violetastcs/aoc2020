const INPUT: &str = include_str!("input.txt");

use std::collections::HashSet;

fn main() {
        println!("-- PART 1 --");

        let questions: usize = INPUT
                .split("\n\n")
                .map(|group| -> HashSet<char> { group
                        .chars()
                        .filter(|c| *c != '\n')
                        .collect() })
                .map(|set| set.iter().count())
                .sum();

        println!("Answer: {}", questions);

        println!("-- PART 2 --");

        let sum: usize = INPUT
                .split("\n\n")
                .map(|group| group
                        .lines()
                        .map(|lines| -> HashSet<char> { lines
                                .chars()
                                .collect() })
                        .reduce(|acc, set| -> HashSet<char> { (&acc & &set)
                                .iter()
                                .map(|c| *c)
                                .collect() })
                        .unwrap()
                        .iter()
                        .count())
                .sum();

        println!("Answer: {}", sum);
}
