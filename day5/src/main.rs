#![feature(saturating_int_impl)]

const INPUT: &str = include_str!("input.txt");

use std::collections::HashSet;
use std::num::Saturating;

fn main() {
        println!("-- PART 1 --");

        let highest_id = INPUT 
                .lines()
                .map(|l| (&l[0..7], &l[7..]))
                .map(|(r, c)| {
                        let r = r.chars();
                        let mut space = (0, 127);

                        for half in r {
                                let size = space.1 - space.0;

                                match half {
                                        // lower
                                        'F' => {
                                                space.1 = space.0 + (size / 2);
                                        },
                                        // upper
                                        'B' => space.0 = space.1 - (size / 2),

                                        _ => unimplemented!()
                                }
                        }

                        (space.0, c)
                })
                .map(|(r, c)| {
                        let c = c.chars();
                        let mut space = (0, 7);

                        for half in c {
                                let size = space.1 - space.0;

                                match half {
                                        // lower
                                        'L' => {
                                                space.1 = space.0 + (size / 2);
                                        },
                                        // upper
                                        'R' => space.0 = space.1 - (size / 2),

                                        _ => unimplemented!()
                                }
                        }

                        (r, space.0)
                })
                .map(|(r, c)| r * 8 + c)
                .max()
                .unwrap();

        println!("Answer: {}", highest_id);

        println!("-- PART 2 --");

        let mut seat_ids: HashSet<_> = INPUT 
                .lines()
                .map(|l| (&l[0..7], &l[7..]))
                .map(|(r, c)| {
                        let r = r.chars();
                        let mut space = (0, 127);

                        for half in r {
                                let size = space.1 - space.0;

                                match half {
                                        // lower
                                        'F' => {
                                                space.1 = space.0 + (size / 2);
                                        },
                                        // upper
                                        'B' => space.0 = space.1 - (size / 2),

                                        _ => unimplemented!()
                                }
                        }

                        (space.0, c)
                })
                .map(|(r, c)| {
                        let c = c.chars();
                        let mut space = (0, 7);

                        for half in c {
                                let size = space.1 - space.0;

                                match half {
                                        // lower
                                        'L' => {
                                                space.1 = space.0 + (size / 2);
                                        },
                                        // upper
                                        'R' => space.0 = space.1 - (size / 2),

                                        _ => unimplemented!()
                                }
                        }

                        (r, space.0)
                })
                .map(|(r, c)| r * 8 + c)
                .collect();

        let max = *seat_ids.iter().max().unwrap();
        let min = *seat_ids.iter().min().unwrap();

        for seat in min..max {
                match (seat_ids.contains(&(seat - 1)),  seat_ids.contains(&seat), seat_ids.contains(&(seat + 1))) {
                        (true, false, true) => { 
                                println!("Answer: {}", seat);
                                break
                        },

                        _ => continue
                }
        }
}
