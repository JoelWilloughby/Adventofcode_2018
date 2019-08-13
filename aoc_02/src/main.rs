
use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

fn num_diffs(s1 : &String, s2: &String) -> usize {
    let mut num : usize = 0;
    let b1 = s1.as_bytes();
    let b2 = s2.as_bytes();

    for i in 0..b1.len() {
        if b1[i] != b2[i] {
            num += 1;
        }
    }

    num
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let v = read(f);

    let mut sum_two : usize = 0;
    let mut sum_three : usize = 0;

    let mut left_sums : Vec<isize> = vec![0; v.len()];
    let mut right_sums : Vec<isize> = vec![0; v.len()];

    for s in v.iter() {
        let mut found = false;
        for s2 in v.iter() {
            if num_diffs(s, s2) == 1 {
                println!("{}", s);
                println!("{}", s2);
                found = true;
                break;
            }
        }

        if found {
            break;
        }
    }

    Ok(())
}
