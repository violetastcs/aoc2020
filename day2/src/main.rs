const INPUT: &str = include_str!("input.txt");

fn validate_part1(pass: &&str) -> bool {
        let split: Vec<_> = pass.split(&['-', ' ', ':'][..])
                .filter(|s| s.len() != 0)
                .collect();

        let (min, max, chr, pass): (usize, usize, char, &str) = (
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
                split[3]
        );

        let count = pass.chars()
                .filter(|c| *c == chr)
                .count();

        count >= min && count <= max
}

fn validate_part2(pass: &&str) -> bool {
        let split: Vec<_> = pass.split(&['-', ' ', ':'][..])
                .filter(|s| s.len() != 0)
                .collect();

        let (fst, snd, chr, pass): (usize, usize, char, &str) = (
                split[0].parse().unwrap(),
                split[1].parse().unwrap(),
                split[2].parse().unwrap(),
                split[3]
        );

        let pass: Vec<_> = pass.chars().collect();

        (pass[fst-1] == chr) ^ (pass[snd-1] == chr)
}

fn main() {
        println!("-- PART 1 --");

        let count = INPUT.lines()
                .filter(validate_part1)
                .count();

        println!("Answer: {}", count);

        println!("-- PART 2 --");

        let count = INPUT.lines()
                .filter(validate_part2)
                .count();

        println!("Answer: {}", count);
}
