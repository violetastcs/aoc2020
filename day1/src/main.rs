use itertools::Itertools;

const INPUT: &str = include_str!("input.txt");

fn main() {
        println!("-- PART 1 --");

        let result = INPUT
                .lines()
                .map(str::parse::<i64>)
                .filter_map(Result::ok)
                .cartesian_product(INPUT
                        .lines()
                        .map(str::parse::<i64>)
                        .filter_map(Result::ok))
                .filter_map(|(n,m)| if n + m == 2020 {
                        Some(n * m)
                } else {
                        None
                })
                .next()
                .unwrap();

        println!("Answer: {}", result);

        println!("-- PART 2 --");

        let result = INPUT
                .lines()
                .map(str::parse::<i64>)
                .filter_map(Result::ok)
                .cartesian_product(INPUT
                        .lines()
                        .map(str::parse::<i64>)
                        .filter_map(Result::ok))
                .cartesian_product(INPUT
                        .lines()
                        .map(str::parse::<i64>)
                        .filter_map(Result::ok))
                .filter_map(|((a,b),c)| if a + b + c == 2020 {
                        Some(a * b * c)
                } else {
                        None
                })
                .next()
                .unwrap();

        println!("Answer: {}", result);
}