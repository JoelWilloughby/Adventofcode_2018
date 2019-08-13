
use std::fs::File;
use std::io::{Read, BufReader, BufRead, Error, ErrorKind};
use std::collections::VecDeque;
use std::collections::HashSet;

fn read<R: Read>(io: R) -> Vec<String> {
    let br = BufReader::new(io);
    br.lines().map(|l| l.unwrap()).collect()
}

#[derive(Debug)]
#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize,  y: usize) -> Self {
        Self {
            x, y,
        }
    }

    fn from_string(s : &str) -> Result<Self, Error> {
        let nums : Vec<&str> = s.split(", ").collect();
        if nums.len() != 2 {
            return Err(Error::new(ErrorKind::InvalidInput, "Invalid input"));
        }

        let x = nums[0].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;
        let y = nums[1].parse().map_err(|e| Error::new(ErrorKind::InvalidInput, e))?;

        Ok(
            Self {
                x, y,
            }
        )
    }
}

#[derive(Debug)]
struct Location {
    point: Point,
    id: Option<char>,
    claimed: bool,
    distance: usize,
}

impl Location {
    fn new(p: &Point) -> Self {
        Self {
            point: *p,
            id: None,
            claimed: false,
            distance: 0,
        }
    }

    fn claim(&mut self, id: Option<char>, distance: usize) -> bool{
        if self.claimed {
            if self.distance > distance {
                // Means this new guy is better
                self.id = id;
                self.distance = distance;
            }
            else if self.distance == distance {
                // Guy is the same, but now don't have one owner to propogate
                if self.id != id {
                    self.id = None;
                }
            }

            return false;
        }
        else {
            self.distance = distance;
            self.id = id;
            self.claimed = true;
            return true;
        }
    }
}

fn make_grid(min_x : usize, min_y : usize, max_x: usize, max_y: usize, locs : &Vec<Point>)
    -> Vec<Vec<Location>> {
    // Initialize grid
    let mut grid: Vec<Vec<Location>> = vec![];
    for i in min_x..=max_x {
        grid.push(vec![]);
        for j in min_y..=max_y {
            grid[i-min_x].push(Location::new(&Point::new(i - min_x, j - min_y)));
        }
    }

    let mut curr_id = 'A';
    for loc in locs.iter() {
        grid[loc.x][loc.y].claim(Some(curr_id), 0);
        curr_id = ((curr_id as u8) + 1) as char; 
    }

    grid
}

fn visit(id: Option<char>, distance: usize, p : &Point, grid: &mut Vec<Vec<Location>>) -> Option<Point> {
    if p.x >= grid.len() {
        return None;
    }
    if p.y >= grid[p.x].len() {
        return None;
    }

    let loc: &mut Location = &mut grid[p.x][p.y];

    if loc.claim(id, distance) {
        return Some(loc.point);
    }

    None
}

fn part_1(v : &Vec<Point>,  grid : &mut Vec<Vec<Location>>) {
    let mut working : VecDeque<Point> = VecDeque::new();

    // Prime the frontier
    for loc in v.iter() {
        working.push_back(*loc);
    }

    // Run the algo, basically a breadth first search
    while !working.is_empty() {
        let curr = working.pop_front().unwrap();
        // Got curr, so we explore neighbros
        let id = grid[curr.x][curr.y].id;
        let distance = grid[curr.x][curr.y].distance + 1;

        let right = Point::new(curr.x + 1, curr.y);
        let up = Point::new(curr.x, curr.y + 1);

        if let Some(x) = visit(id, distance, &right, grid) {
            working.push_back(x);
        }
        if let Some(x) = visit(id, distance, &up, grid) {
            working.push_back(x);
        }

        if curr.x > 0 {
            let left = Point::new(curr.x - 1, curr.y);
            if let Some(x) = visit(id, distance, &left, grid) {
                working.push_back(x);
            }
        }

        if curr.y > 0 {
            let down = Point::new(curr.x, curr.y - 1);
            if let Some(x) = visit(id, distance, &down, grid) {
                working.push_back(x);
            }
        }

    }

    let mut map : [usize; 256] = [0; 256];

    for row in grid.iter() {
        for loc in row.iter() {
            if let Some(x) = loc.id {
                map[x as usize] += 1;
                print!("{}", x);
            }
            else {
                print!(".");
            }
        }
        println!("");
    }

    // Disqualify those touching the edges
    for loc in grid[0].iter() {
        if let Some(x) = loc.id {
            map[x as usize] = 0;
        }
    }
    for loc in grid[grid.len()-1].iter() {
        if let Some(x) = loc.id {
            map[x as usize] = 0;
        }
    }
    for row in grid.iter() {
        if let Some(x) = row[0].id {
            map[x as usize] = 0;
        }
        if let Some(x) = row[row.len()-1].id {
            map[x as usize] = 0;
        }
    }

    let mut max = 0;
    for val in map.iter() {
        if *val > max {
            max = *val;
        }
    }

    println!("Max val is {}", max);
}


struct Point2 {
    x : isize,
    y : isize,
    xdir : isize,
    xi : usize,
    ydir : isize,
    yi : usize,

    xdist : usize,
    ydist : usize,
    total : usize,
}

impl Point2 {
    fn xmove(&self, xs : &Vec<isize>) -> Self {
        let new_x = self.x + self.xdir;
        let mut new_xi = self.xi;
        let mut new_xdist = self.xdist;
        if self.xdir > 0 {
            while xs[new_xi] < new_x {
                new_xi += 1;
                new_xdist += 2;
            };
        } else {
            while xs[new_xi] > new_x {
                new_xi -= 1;
                new_xdist += 2;
            }
        };

        let new_total = self.total + new_xdist;

        Self {
            x : new_x,
            y : self.y,
            xdir : self.xdir,
            xi : new_xi,
            ydir : self.ydir,
            yi : self.yi,
            xdist : new_xdist,
            ydist : self.ydist,
            total : new_total,
        }
    }

    fn ymove(&self, ys : &Vec<isize>) -> Self {
        let new_y = self.y + self.ydir;
        let mut new_yi = self.yi;
        let mut new_ydist = self.ydist;
        if self.ydir > 0 {
            while ys[new_yi] < new_y {
                new_yi += 1;
                new_ydist += 2;
            };
        } else {
            while ys[new_yi] > new_y {
                new_yi -= 1;
                new_ydist += 2;
            }
        };

        let new_total = self.total + new_ydist;

        Self {
            x : self.x,
            y : new_y,
            xdir : self.xdir,
            xi : self.xi,
            ydir : self.ydir,
            yi : new_yi,
            xdist : self.xdist,
            ydist : new_ydist,
            total : new_total,
        }
    }
}


fn part_2(v: &Vec<Point>, target: isize) {
    // Idea is to start from the median, which is the lowest distance point. From there, we walk in
    // All four directions and do a sort of breadth-first search, keeping track of the current
    // distance at each point.
    //                ^
    //                |   ^
    //             <- .   |
    //         <- .   X   . ->
    //            |   . ->
    //            v   |
    //                v
    let mut xs: Vec<_> = v.iter().map(|p| p.x as isize).collect();
    let mut ys: Vec<_> = v.iter().map(|p| p.y as isize).collect();

    xs.sort();
    ys.sort();

    let xmh = xs.len() / 2;
    let xml = (xs.len() - 1) / 2;
    let xmed = (xs[xml] + xs[xmh]) / 2;
    let ymh = ys.len() / 2;
    let yml = (ys.len() - 1) / 2;
    let ymed = (ys[yml] + ys[ymh]) / 2;

    let x_dist = xs.iter().fold(0, |a, x| a + ((*x)-xmed).abs());
    let y_dist = ys.iter().fold(0, |a, y| a + ((*y)-ymed).abs());
    let rem_: isize = target - x_dist - y_dist;

    let mut working : VecDeque<Point2> = VecDeque::new();

    let mut median = Point2 {
        x : xmed,
        y : ymed,
        xdir : 1,
        xi : xmh,
        ydir : 1,
        yi : ymh,
        // 0 because our dataset happens to be even sized
        xdist : 0,
        ydist : 0,
        total : 0,
    };

    working.push_back(median.xmove(&xs));
    median.xdir = -1;
    median.xi = xml;
    working.push_back(median.ymove(&ys));
    median.ydir = -1;
    median.yi = yml;
    working.push_back(median.xmove(&xs));
    median.xdir = 1;
    median.xi = xmh;
    working.push_back(median.ymove(&ys));

    let mut accepted = HashSet::new();
    accepted.insert((xmed, ymed));

    while !working.is_empty() {
        let curr = working.pop_front().unwrap();
        //println!("Curr is {},{},{},{} -> {}", curr.x, curr.y, curr.xdir, curr.ydir, curr.total);

        if curr.total >= rem_ as usize {
            continue;
        }

        if accepted.contains(&(curr.x, curr.y)) {
            continue;
        }

        working.push_back(curr.xmove(&xs));
        working.push_back(curr.ymove(&ys));

        accepted.insert((curr.x, curr.y));
    }

    println!("Accepted size is {}", accepted.len());

    //for point in accepted.iter() {
    //    println!("({}, {})", point.0, point.1);
    //}
}

fn main() -> std::io::Result<()> {
    let f = File::open("./input.txt")?;
    let mut v: Vec<Point> = read(f).iter().map(|s| Point::from_string(s).unwrap()).collect();

    // Find bounds
    let mut min_x = v[0].x;
    let mut min_y = v[0].y;
    let mut max_x = v[0].x;
    let mut max_y = v[0].y;
    for point in v.iter() {
        max_x = std::cmp::max(point.x, max_x);
        max_y = std::cmp::max(point.y, max_y);
        min_x = std::cmp::min(point.x, min_x);
        min_y = std::cmp::min(point.y, min_y);
    }

    v = v.iter().map(|point| Point::new(point.x - min_x, point.y - min_y)).collect();

    println!("Grid dims: {} -> {} x {} -> {}", min_x, max_x, min_y, max_y);

    // Initialize grid
    let mut grid = make_grid(min_x, min_y, max_x, max_y, &v);

    part_1(&v, &mut grid);
    part_2(&v, 10000);

    Ok(())
}
