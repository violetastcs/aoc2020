const INPUT: &str = include_str!("input.txt");

#[derive(Default)]
struct Passport<'a> {
        byr: Option<&'a str>,
        iyr: Option<&'a str>,
        eyr: Option<&'a str>,
        hgt: Option<&'a str>,
        hcl: Option<&'a str>,
        ecl: Option<&'a str>,
        pid: Option<&'a str>,
        cid: Option<&'a str>
}

impl<'a> Passport<'a> {
        fn entry(&mut self, key: &'a str) -> &mut Option<&'a str> {
                match key {
                        "byr" => &mut self.byr,
                        "iyr" => &mut self.iyr,
                        "eyr" => &mut self.eyr,
                        "hgt" => &mut self.hgt,
                        "hcl" => &mut self.hcl,
                        "ecl" => &mut self.ecl,
                        "pid" => &mut self.pid,
                        "cid" => &mut self.cid,

                        _ => unimplemented!()
                }
        }

        fn from_str(s: &'a str) -> Passport<'a> {
                let mut passport = Passport::default();

                let kvs = s.split(&[' ', '\n'][..])
                        .filter(|kv| kv.len() != 0);

                for kv in kvs {
                        let mut kv = kv.split(':');

                        let (key, val) = (
                                kv.next().unwrap(),
                                kv.next().unwrap()
                        );

                        *passport.entry(key) = Some(val);
                }

                passport
        }

        fn is_complete(&self) -> bool {
                self.byr
                        .and(self.iyr)
                        .and(self.eyr)
                        .and(self.hgt)
                        .and(self.hcl)
                        .and(self.ecl)
                        .and(self.pid)
                        .is_some()
        }

        fn is_valid(&self) -> bool {
                if self.is_complete() {
                        let byr = self.byr
                                .unwrap()
                                .parse::<u16>()
                                .unwrap_or(0);

                        let iyr = self.iyr
                                .unwrap()
                                .parse::<u16>()
                                .unwrap_or(0);

                        let eyr = self.eyr
                                .unwrap()
                                .parse::<u16>()
                                .unwrap_or(0);

                        let hgt = self.hgt.unwrap();

                        let (hgt_max, hgt_min) = match &hgt[hgt.len() - 2..] {
                                "in" => (76, 59),
                                "cm" => (193, 150),

                                _ => return false
                        };

                        let hgt: u8 = (&hgt[..hgt.len() - 2]).parse()
                                .unwrap();

                        let mut hcl = self.hcl.unwrap()
                                .chars();

                        let hcl_valid = hcl.next() == Some('#')
                                && hcl
                                        .filter(|c| matches!(c, 'a'..='f' | '0'..='9'))
                                        .count() == 6;

                        let ecl = self.ecl.unwrap();

                        let pid = self.pid.unwrap();

                        (byr >= 1920 && byr <= 2002) &&
                        (iyr >= 2010 && iyr <= 2020) &&
                        (eyr >= 2020 && eyr <= 2030) &&

                        (hgt >= hgt_min && hgt <= hgt_max) &&
                        
                        (hcl_valid) &&

                        (matches!(ecl, 
                                "amb" | "blu" | "brn" | 
                                "gry" | "grn" | "hzl" | 
                                "oth"
                        )) &&

                        (pid.len() == 9 && pid.parse::<i32>().is_ok())
                } else {
                        false
                }
        }
}

fn main() {
        println!("-- PART 1 --");

        let count = INPUT
                .split("\n\n")
                .map(Passport::from_str)
                .filter(Passport::is_complete)
                .count();

        println!("Answer: {}", count);

        println!("-- PART 2 --");

        let count = INPUT
                .split("\n\n")
                .map(Passport::from_str)
                .filter(Passport::is_valid)
                .count();

        println!("Answer: {}", count);
}