
fn make_grid(sn: isize, rows: usize, cols: usize) -> Vec<Vec<isize>> {
    let mut ret = vec![];
    for row in 0..rows {
        let mut rowv = vec![];
        for col in 0..cols{
            let mut val : isize = row as isize + 11;
            val *= (col + 1) as isize;
            val += sn;
            val *= (row as isize) + 11;
            val /= 100;
            val %= 10;
            val -= 5;
            rowv.push(val);
        }
        ret.push(rowv);
    }

    ret
}

fn zip_rows(grid: &Vec<Vec<isize>>, offset: usize) -> Vec<Vec<isize>> {
    let mut ret = vec![];
    for row in grid.iter() {
        let mut rowv = vec![];
        let mut running_sum = 0;
        for cell in row.iter().take(offset) {
            running_sum += *cell;
        }
        for i in 0..(row.len() - offset) as usize {
            running_sum += row[i + offset];
            rowv.push(running_sum);
            running_sum -= row[i];
        }
        ret.push(rowv);
    }
    ret
}

fn zip_cols(grid: &Vec<Vec<isize>>, offset: usize) -> Vec<Vec<isize>> {
    let mut ret = vec![];
    for _ in 0..(grid.len() - offset) as usize {
        ret.push(vec![]);
    }
    for j in 0..(grid[0].len()) as usize {
        let mut running_sum = 0;
        for i in 0..offset {
            running_sum += grid[i][j];
        }
        for i in 0..(grid.len() - offset) as usize {
            running_sum += grid[i+offset][j];
            ret[i].push(running_sum);
            running_sum -= grid[i][j];
        }
    }

    ret
}

fn main() {
    let grid = make_grid(1133,300,300);

    let mut max = isize::min_value();
    let mut coord = (0, 0);
    let mut size = 0;

    for s in 0..300 {
        let grid2 = zip_rows(&grid, s);
        let grid3 = zip_cols(&grid2, s);
        for (i, row) in grid3.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if *cell > max {
                    max = *cell;
                    coord = (i, j);
                    size = s;
                }
            }
        }
    }

    println!("\nMax is {}, coord is {:?}, size is {}", max, coord, size)
}
