use std::fs::File;
use std::io::{Read, BufReader, BufRead};

fn read<R: Read>(io: R) -> Vec<usize> {
    let mut br = BufReader::new(io);
    let mut temp = String::new();
    br.read_line(&mut temp);
    temp.trim().split(char::is_whitespace).map(|s| s.parse().unwrap()).collect()
}

fn traverse<'a, I> (iter: &mut I) -> usize
    where I: Iterator<Item = &'a usize> {
    let num_children = *iter.next().unwrap();
    let num_meta = *iter.next().unwrap();

    let mut sums = vec![];

    for i in 0..num_children {
        sums.push(traverse(iter));
    }

    let mut sum = 0;
    for i in 0..num_meta {
        if num_children == 0 {
            sum += *iter.next().unwrap();
        } else {
            let index = *iter.next().unwrap();
            sum += *sums.get(index - 1).unwrap_or(&0);
        }
    }

    sum
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let mut ins : Vec<_> = read(f);

    for i in ins.iter() {
        println!("{}", i);
    }

    let sum = traverse(&mut ins.iter());

    println!("Sum is {}", sum);

    Ok(())
}
