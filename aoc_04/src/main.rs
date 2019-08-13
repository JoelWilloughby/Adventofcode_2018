
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Error, ErrorKind};
use std::cmp::Ordering;
use std::collections::HashMap;
use regex::Regex;

#[macro_use]
extern crate lazy_static;

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

#[derive(Eq, PartialEq)]
enum EventType {
    Sleep,
    Wake,
    ShiftChange (usize),
}

#[derive(Eq)]
struct Event {
    event : EventType,
    time : usize,
    minutes : usize,
}


impl Event {
    fn convert_time(year : usize, month: usize, day: usize, hour: usize, minute: usize) -> usize {
        const mins_per_hour: usize = 60;
        const mins_per_day: usize = mins_per_hour * 24;
        const mins_per_month: usize = mins_per_day * 31;
        const mins_per_year: usize = mins_per_month * 12;
        return year * mins_per_year + month * mins_per_month + day * mins_per_day + hour * mins_per_hour + minute;
    }

    fn from_string(s: &String) -> Result<Self, Error> {
        // Drastically reduces runtime to not have to compile this thing everytime
        lazy_static! {
            static ref event_re: Regex = Regex::new(r"\[(\d+)-(\d+)-(\d+) (\d{2}):(\d{2})\] * (.*)$").unwrap();
            static ref wake_re: Regex = Regex::new(r"wakes up").unwrap();
            static ref sleep_re: Regex = Regex::new(r"falls asleep").unwrap();
            static ref guard_re: Regex = Regex::new(r"Guard #(\d+) begins shift").unwrap();
        }

        let caps = event_re.captures(s)
            .ok_or(Error::new(ErrorKind::InvalidInput, "Not a valid event"))?;

        // println!("Making new claim {}", s);

        let year = caps[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let month = caps[2].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let day = caps[3].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let hour = caps[4].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let minute = caps[5].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let time = Self::convert_time(year, month, day, hour, minute);

        if wake_re.is_match(&caps[6]) {
            return Ok(Self {
                time: time,
                minutes: minute,
                event: EventType::Wake,
            });
        }

        if sleep_re.is_match(&caps[6]) {
            return Ok(Self {
                time: time,
                minutes: minute,
                event: EventType::Sleep,
            });
        }

        if let Some(guard) = guard_re.captures(&caps[6]) {
            let guard_id = guard[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
            return Ok(Self {
                time: time,
                minutes: minute,
                event: EventType::ShiftChange(guard_id),
            });
        }

        Err(Error::new(ErrorKind::InvalidInput, "Unknown Event Type"))
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            EventType::Wake => write!(f, "Wakes up"),
            EventType::Sleep => write!(f, "Sleepy Bye"),
            EventType::ShiftChange(id) => write!(f, "Guard {} comes in", id)
        }
    }
}

impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -- {}", self.time, self.event)
    }
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let mut events : Vec<_> = read(f).iter().map(|s| Event::from_string(s).unwrap()).collect();

    events.sort_unstable();
    let mut guards : HashMap<usize, Vec<(usize, usize)>> = HashMap::new();

    let mut current_guard : usize = 0;
    match events[0].event {
        EventType::ShiftChange(id) => {
            guards.insert(id, vec![]);
            current_guard = id;
        },
        _ => return Err(Error::new(ErrorKind::InvalidInput, "First event is not a shift change")),
    };

    let mut fall_asleep : Option<usize> = None;

    for event in events.iter() {
        println!("Now at {}", event);
        match event.event {
            EventType::Wake => {
                let initial_sleep = fall_asleep.unwrap();
                let sleepy_time = event.minutes - initial_sleep;
                guards.get_mut(&current_guard).unwrap().push((initial_sleep, sleepy_time));
                fall_asleep = None;
            },
            EventType::Sleep => {
                fall_asleep = Some(event.minutes);
            },
            EventType::ShiftChange(id) =>  {
                guards.entry(id).or_insert_with(|| vec!());
                current_guard = id;
            },
        };
    }

    let mut max_guard = 0;
    let mut max_num = 0;
    for (guard, sleeps) in guards.iter() {
        let mut sum = 0;
        println!("--------------- {} ------------------", guard);
        for sleep in sleeps.iter() {
            sum += sleep.1;
            println!("{},{}", sleep.0, sleep.1);
        }
        println!("");
        if sum > max_num {
            max_num = sum;
            max_guard = *guard;
        }
    }

    println!("sleepy guard is {}", max_guard);
    let guard = guards.get(&max_guard).unwrap();
    let mut mins: [usize; 60] = [0;60];
    for (min, length) in guard.iter() {
        for i in (*min)..(*min+*length) {
            mins[i] += 1
        }
    }

    let mut max = 0;
    let mut max_val = mins[0];
    for (index, min) in mins.iter().enumerate() {
        if *min > max_val {
            max = index;
            max_val = *min;
        }
    }

    println!("sleepy minute is {}", max);

    let mut max = 0;
    let mut max_guard : usize = 0;
    let mut max_minute : usize = 0;
    let mut min_with_guards: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 60];
    for (guard, sleeps) in guards.iter() {
        for (min, length) in sleeps.iter() {
            for i in (*min)..(*min+*length) {
                let mut count = min_with_guards[i].entry(*guard).or_insert(0);
                *count += 1;
                if *count > max {
                    max = *count;
                    max_guard = *guard;
                    max_minute = i;
                }
            }
        }
    }

    println!("Guard: {}, Min: {}, Count: {}", max_guard, max_minute, max);

    Ok(())
}
