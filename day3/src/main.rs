const INPUT: &str = include_str!("input.txt");

fn calc_slope((dx, dy): (usize, usize)) -> usize {
        INPUT
                .lines()
                .map(|l| l.chars().cycle())
                .enumerate()
                .step_by(dy)
                .filter_map(|(i, mut p)| p.nth((i / dy) * dx))
                .filter(|c| *c == '#')
                .count()
}

fn main() {
        println!("-- PART 1 --");

        let count = calc_slope((3, 1));
        
        println!("Answer: {}", count);

        println!("-- PART 2 --");

        let slopes = [(1,1), (3,1), (5,1), (7,1), (1,2)];

        let product: usize = slopes
                .into_iter()
                .map(calc_slope)
                .product();

        println!("Answer: {}", product);
}
