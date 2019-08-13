use regex::Regex;
use std::io::{Read, BufRead, BufReader};
use std::fs::File;
use std::fmt::{Display, Formatter};

fn parse_input<R: Read>(io: R) -> (String, Vec<(String, String)>) {
    let br = BufReader::new(io);
    let lines: Vec<_> = br.lines().map(|l| l.unwrap()).collect();

    let in_re = Regex::new(r"initial state: ([#.]*)").unwrap();
    let map_re = Regex::new(r"([#.]*) => ([#.])").unwrap();
    let input = in_re.captures(&lines[0]).unwrap().get(1).unwrap().as_str().to_string();

    let mut rules = vec![];

    for line in lines.iter().skip(2) {
        let caps = map_re.captures(line).unwrap();
        rules.push((caps.get(1).unwrap().as_str().to_string(), caps.get(2).unwrap().as_str().to_string()));
    }

    (input, rules)
}

#[derive(Debug)]
struct Plants {
    rules: usize,
    state: Vec<bool>,
    offset: isize,
}

impl Plants {
    fn from_input(state: String, rules: Vec<(String, String)>) -> Self {
        let mut init = vec![];
        let mut found_plant = false;
        for c in state.chars() {
            if c == '#' {
                init.push(true);
                found_plant = true;
            } else if c == '.' {
                if found_plant {
                    init.push(false);
                }
            } else {
                panic!("Invalid input");
            }
        }

        let mut da_rules = 0x0;

        for (rule, val) in rules.iter() {
            let mut r = 0;
            for c in rule.chars() {
                r <<= 1;
                if c == '#' {
                    r |= 1;
                } else if c != '.' {
                    panic!("Invalid input");
                }
            }

            if val.len() > 1 {
                panic!("Invalid input");
            }

            let c = val.chars().next().unwrap();
            if c == '#' {
                da_rules |= 1 << r
            } else if c != '.' {
                panic!("Invalid input");
            }
        }

        Self {
            rules: da_rules,
            state: init,
            offset: 0,
        }
    }

    fn grow(&mut self) {
        let mut curr: usize = 0;
        let mut next_state = vec![];
        let mut next_offset = self.offset - 3;
        let mut plant_found = false;
        // Need these for reasons
        self.state.push(false);
        self.state.push(false);
        self.state.push(false);
        self.state.push(false);
        for plant in self.state.iter() {
            let next = self.rules & (1 << curr);
            if next != 0{
                next_state.push(true);
                plant_found = true;
            } else if plant_found {
                next_state.push(false);
            } else {
                next_offset += 1;
            }

            curr <<= 1;
            // Mod it down to 0 -> 31
            curr &= 0x1f;
            curr |= if *plant { 1 } else { 0 };
        }

        self.state = next_state;
        self.offset = next_offset;
    }

    fn count(&self) -> isize {
        let mut count = 0;
        for (i, s) in self.state.iter().enumerate() {
            if *s {
                count += i as isize + self.offset;
            }
        }

        count
    }
}

impl Display for Plants {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#x} {} ", self.rules, self.offset);
        for s in self.state.iter() {
            if *s {
                write!(f, "#");
            } else {
                write!(f, ".");
            }
        }

        Ok(())
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let (input, rules) = parse_input(f);

    let mut plants = Plants::from_input(input, rules);
    println!("{}", plants);

    for i in 0..20 {
        plants.grow();
        println!("{}", plants);
    }

    println!("{}", plants.count());

    for i in 0..80 {
        plants.grow();
        println!("{}", plants);
    }

    // Just looking for where the pattern repeats itself
    plants.offset = 50_000_000_000 - 91 + 49;

    println!("{}", plants.count());

    Ok(())
}
