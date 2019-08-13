
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Error, ErrorKind};

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

fn reduce(vec: &Vec<char>, ignore: Option<char>) -> Vec<char> {
    let mut working : Vec<char> = vec![];
    let mut top : Option<char> = None;

    let ig_ch = ignore.unwrap_or('\0');

    for ch_ref in vec.iter() {
        let ch = *ch_ref;
        if ch == ig_ch || ch == ig_ch.to_ascii_uppercase() {
            continue;
        }
        match top {
            Some(ch_val) => {
                let val = ch_val as isize;
                let curr_val = ch as isize;
                if val - 32 == curr_val || val + 32 == curr_val {
                    // Pop top
                    working.pop();
                    if !working.is_empty() {
                        top = Some(working[working.len() - 1]);
                    }
                    else {
                        top = None;
                    }
                }
                else {
                    working.push(ch);
                    top = Some(ch);
                }
            }
            None => {
                top = Some(ch);
                working.push(ch);
            }
        }
    }

    working
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let mut v = read(f);
    assert!(v.len() == 1);

    let mut s : Vec<char> = v[0].chars().collect();
    let mut reduced = reduce(&s, None);

    println!("Original length is {}", s.len());
    println!("Length after initial reduction is {}", reduced.len());

    let mut min = reduced.len();
    let mut min_char = 'a';
    for ch in (b'a'..=b'z').map(char::from) {
        let reduced_by_ch = reduce(&reduced, Some(ch));
        if reduced_by_ch.len() < min {
            min = reduced_by_ch.len();
            min_char = ch;
        }
        println!("On letter {}, len: {}", ch, reduced_by_ch.len());
    }

    println!("Best reduction is {} at {}", min_char, min);

    Ok(())
}
