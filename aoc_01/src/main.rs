
use std::fs::File;
use std::io::{Read, Error, BufReader, BufRead, ErrorKind};

use bit_vec::BitVec;

fn read<R: Read>(io: R) -> Result<Vec<i32>, Error> {
    let br = BufReader::new(io);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

struct BitThing {
    bits : BitVec,
}

impl BitThing {
    fn new() -> Self {
        Self {
            bits: BitVec::new()
        }
    }

    fn get_index(i: i32) -> usize {
        let mut ret : usize = (2 * i.abs()) as usize;
        if i < 0 {
            ret += 1;
        }

        ret
    }

    fn insert(&mut self, val: i32) -> bool {
        let index = Self::get_index(val);

        if self.bits.len() <= index {
            let mut extra = BitVec::from_elem(index - self.bits.len() + 1, false);
            self.bits.append(&mut extra);
        }

        if self.bits.get(index).unwrap() {
            return true;
        }

        self.bits.set(index, true);

        false
    }
}

fn loop_once(bits: &mut BitThing, vec : &Vec<i32>, init: i32) -> (bool, i32) {
    let mut found = false;
    let mut sum = init;
    for val in vec.iter() {
        if bits.insert(sum) {
            found = true;
            break;
        }

        sum += val;
    }

    (found, sum)
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let v = read(f)?;

    let mut bits = BitThing::new();

    let mut sumfound = (false, 0);

    while !sumfound.0 {
        sumfound = loop_once(&mut bits, &v, sumfound.1);
    }

    println!("Val is {}", sumfound.1);

    Ok(())
}
