use std::fs::File;
use std::io::{Read, BufReader, BufRead, ErrorKind, Error, stdin, stdout, Write};
use regex::Regex;

#[macro_use]
extern crate lazy_static;

#[derive(Debug)]
struct Star {
    pos_x : isize,
    pos_y : isize,
    vel_x : isize,
    vel_y : isize,
}

impl Star {
    fn from_string(s : &String) -> Result<Self, Error> {
        lazy_static! {
            static ref in_re: Regex = Regex::new(r"position=< *(-?\d+), *(-?\d+)> velocity=< *(-?\d), *(-?\d)>$").unwrap();
        }

        let caps = in_re.captures(s).ok_or(Error::new(ErrorKind::InvalidInput, "Not a valid star"))?;
        let pos_x = caps[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let pos_y = caps[2].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let vel_x = caps[3].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let vel_y = caps[4].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        Ok(
            Self {
                pos_x,
                pos_y,
                vel_x,
                vel_y,
            }
        )
    }

    fn walk(&mut self, n: isize) {
        self.pos_x += n*self.vel_x;
        self.pos_y += n*self.vel_y;
    }
}

fn read<R: Read>(io: R) -> Vec<Star> {
    let mut br = BufReader::new(io);
    br.lines().map(|x| Star::from_string(&x.unwrap()).unwrap()).collect()
}

fn stats(stars: &Vec<Star>) -> (f32, f32) {
    let mut x_sum : f32 = 0.0;
    let mut y_sum : f32 = 0.0;
    for star in stars.iter() {
        x_sum += (star.pos_x as f32) / (star.vel_x as f32);
        y_sum += (star.pos_y as f32) / (star.vel_y as f32);
    }

    (x_sum / stars.len() as f32, y_sum / stars.len() as f32)

}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let mut ins : Vec<_> = read(f);

    for star in ins.iter() {
        println!("{:?}", star);
    }

    loop {
        const half_size : usize = 200;
        let mut grid: [[bool; 2 * half_size + 1]; 2*half_size + 1] = [[false; 2*half_size + 1]; 2*half_size + 1];

        for star in ins.iter() {
            if star.pos_x < half_size as isize && star.pos_x > half_size as isize * -1 && star.pos_y < half_size as isize && star.pos_y > half_size as isize * -1 {
                grid[(star.pos_y + half_size as isize) as usize][(star.pos_x + half_size as isize) as usize] = true;
            }
        }

        for line in grid.iter() {
            for star in line.iter() {
                if *star {
                    print!("*");
                }
                else {
                    print!(" ");
                }
            }
            println!();
        }

        println!("Current stats: {:?}", stats(&ins));
        print!("Choice -- ");
        stdout().flush();
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Did not enter a correct string");
        let dist : isize = s.trim().parse().unwrap();
        for star in ins.iter_mut() {
            star.walk(dist);
        }
    }


    Ok(())
}