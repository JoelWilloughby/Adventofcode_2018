use std::fs::File;
use std::io::{Read, BufReader, BufRead};
use regex::Regex;
use std::collections::HashMap;
use std::collections::BinaryHeap;
use std::cmp::{Ord, Ordering, PartialOrd, Reverse};

#[macro_use]
extern crate lazy_static;

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

// Drastically reduces runtime to not have to compile this thing everytime
lazy_static! {
    static ref parse_re: Regex = Regex::new(r"Step ([a-zA-Z]) must be finished before step ([a-zA-Z]) can begin.").unwrap();
}

fn from_string(s: &String) -> (char, char) {
    let caps = parse_re.captures(s).unwrap();

    // println!("Making new claim {}", s);

    let from = caps[1].chars().next().unwrap();
    let to = caps[2].chars().next().unwrap();

    (from, to)
}

#[derive(Copy, Clone)]
enum State {
    Nothing,
    Queued,
    Done,
}

fn depth_first(start: char, graph : &HashMap<char, Vec<char>>, state : &mut [State], done: &mut Vec<char>) {
    match state[start as usize] {
        State::Queued => return (),
        State::Done => return (),
        _ => {},
    }

    state[start as usize] = State::Queued;

    for neigh in graph[&start].iter() {
        depth_first(*neigh, graph, state, done);
    }

    done.push(start);
    state[start as usize] = State::Done;
}

fn part_1(graph: &mut HashMap<char, Vec<char>>) {
    let mut states : [State; 256] = [State::Nothing; 256];
    let mut order : Vec<char> = vec![];

    // Get the list of keys
    let mut tasks : Vec<char> = graph.keys().into_iter().map(|x| *x).collect();
    tasks.sort();
    tasks.reverse();

    for task in tasks.iter() {
        graph.get_mut(task).unwrap().sort();
        graph.get_mut(task).unwrap().reverse();
    }

    for task in tasks.iter() {
        depth_first(*task, &graph, &mut states, &mut order);
    }

    for task in order.iter().rev() {
        print!("{}", task);
    }

    println!();
}

#[derive(Eq, PartialEq)]
struct Event {
    time : usize,
    task : char,
}

impl Ord for Event {
    // Flipped for use in Max BinaryHeap
    fn cmp(&self, other: &Event) -> Ordering {
        other.time.cmp(&self.time)
            .then_with(|| other.task.cmp(&self.task))
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part_2(graph: &mut HashMap<char, Vec<char>>, rev_graph: &mut HashMap<char, Vec<char>>) {
    let mut pq = BinaryHeap::new();
    let mut to_q : BinaryHeap<Reverse<char>> = BinaryHeap::new();
    let mut workers_avail: usize = 5;
    let mut counts: [usize; 256] = [0; 256];

    for (task, neighs) in graph.iter() {
        counts[*task as usize] = neighs.len();
        if neighs.len() == 0 {
            to_q.push(Reverse(*task));
        }
    }

    while workers_avail != 0 && !to_q.is_empty() {
        let task = to_q.pop().unwrap().0;
        pq.push(Event {time: 60 + task as usize - 'A' as usize + 1, task: task});
        workers_avail -= 1;
    }

    let mut curr_time: usize = 0;

    while !pq.is_empty() {
        let event = pq.pop().unwrap();
        workers_avail += 1;
        curr_time = event.time;
        // println!("Task {} finished at time {}, workers: {}", event.task, event.time, workers_avail);

        // Update the counts based on this task completing
        for neigh in rev_graph[&event.task].iter() {
            counts[*neigh as usize] -= 1;
            if counts[*neigh as usize] == 0 {
                // This was the last neighbor, we can now push.
                to_q.push(Reverse(*neigh));
            }
        }

        while workers_avail != 0 && !to_q.is_empty() {
            let task = to_q.pop().unwrap().0;
            pq.push(Event {time: curr_time + 60 + (task as usize) - ('A' as usize) + 1, task: task});
            workers_avail -= 1;
            // println!("Worker taking job {} at time {}, rem: {}", task, curr_time, workers_avail);
        }
    }

    println!("Time taken: {}", curr_time);
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let edges : Vec<_> = read(f).iter().map(|s| from_string(s)).collect();

    let mut graph = HashMap::new();
    let mut rev_graph = HashMap::new();

    for edge in edges {
        let tos = graph.entry(edge.0).or_insert(vec![]);
        let froms = rev_graph.entry(edge.1).or_insert(vec![]);
        tos.push(edge.1);
        froms.push(edge.0);
        {
            graph.entry(edge.1).or_insert(vec![]);
            rev_graph.entry(edge.0).or_insert(vec![]);
        }
    }

    part_1(&mut graph);
    part_2(&mut rev_graph, &mut graph);

    Ok(())
}
