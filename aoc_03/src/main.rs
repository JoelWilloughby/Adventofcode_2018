
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Error, ErrorKind};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

struct Claim {
    id : usize,
    x : isize,
    y : isize,
    w : isize,
    h : isize,
}

// Drastically reduces runtime to not have to compile this thing everytime
lazy_static! {
    static ref parse_re: Regex = Regex::new(r"#(\d+) *@ *(\d+),(\d+): *(\d+)x(\d+)").unwrap();
}

impl Claim {

    fn from_string(s: &String) -> Result<Self, Error> {
        let caps = parse_re.captures(s).unwrap();

        // println!("Making new claim {}", s);

        let id = caps[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let x = caps[2].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let y = caps[3].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let w = caps[4].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let h = caps[5].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        Ok(Claim {
            id: id,
            x: x,
            y: y,
            w: w,
            h: h,
        })
    }
}

impl std::fmt::Display for Claim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.x, self.y, self.w, self.h)
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let claims : Vec<_> = read(f).iter().map(|s| Claim::from_string(s).unwrap()).collect();

    let mut min_x = 1000000;
    let mut max_x = 0;
    let mut min_y = 1000000;
    let mut max_y = 0;

    for claim in claims.iter() {
        if claim.x < min_x {
            min_x = claim.x;
        }
        if claim.y < min_y {
            min_y = claim.y;
        }
        if claim.x + claim.w > max_x {
            max_x = claim.x + claim.w;
        }
        if claim.y + claim.h > max_y {
            max_y = claim.y + claim.h;
        }
    }


    // Brute force!!
    let x_width = (max_x - min_x) as usize;
    let y_width = (max_y - min_y) as usize;

    let mut grid: Vec<Vec<(Vec<usize>, usize)> > = vec![vec![(vec![0;0], 0); y_width]; x_width];
    let mut sum = 0;

    for (index,claim) in claims.iter().enumerate() {
        for i in 0..claim.w {
            for j in 0..claim.h {
                grid[(i + claim.x - min_x) as usize][(j + claim.y - min_y) as usize].0.push(index);
                grid[(i + claim.x - min_x) as usize][(j + claim.y - min_y) as usize].1 += 1;
            }
        }
    }

    for (index,claim) in claims.iter().enumerate() {
        let mut oneandonly = true;
        for i in 0..claim.w {
            for j in 0..claim.h {
                if grid[(i + claim.x - min_x) as usize][(j + claim.y - min_y) as usize].0.len() != 1 {
                    oneandonly = false;
                }
            }
        }
        if oneandonly {
            println!("One and only!: {}", claim.id);
        }
    }

    for col in grid.iter() {
        for cell in col.iter() {
            if (*cell).1 > 1 {
                sum += 1;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
