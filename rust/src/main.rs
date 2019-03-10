use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Problem {
    data: [[u8; 9]; 9],
    options: [[[u8; 9]; 9]; 9],
    optcounts: [[u8; 9]; 9],
}

struct EmptyCells<'a> {
    problem: &'a Problem,
    curx: usize,
    cury: usize
}

impl<'a> EmptyCells<'a> {
    fn new(problem: &Problem) -> EmptyCells {
        EmptyCells{problem, curx: 0, cury: 0}
    }
}

impl<'a> Iterator for EmptyCells<'a> {
    type Item = (usize, usize);
    fn next(&mut self) -> Option<(usize, usize)> {
        while self.curx != 9 {
            let res = (self.curx, self.cury);
            let val = self.problem.get(self.curx, self.cury);
            // println!("TRYING {},{}", self.curx, self.cury);
            self.cury += 1;
            if self.cury == 9 {
                self.cury = 0;
                self.curx += 1;
            }
            if val == 0 {
                return Some(res);
            }
        }
        None
    }
}

impl Problem {
    fn new() -> Problem {
        Problem {
            data: [[0; 9]; 9],
            options: [[[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9]; 9],
            optcounts: [[9; 9]; 9],
        }
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        self.data[x][y]
    }

    fn set(&mut self, x: usize, y: usize, value: u8) -> Result<(), String> {
        let mut coords = [(0, 0); 9*3];
        let mut curcoord = 0;
        // verify by column
        for ty in 0..9 {
            coords[curcoord] = (x, ty);
            curcoord += 1;
            if self.get(x, ty) == value {
                return Err(format!("Value {} is already in the column {}", value, x));
            }
        }
        // verify by row
        for tx in 0..9 {
            coords[curcoord] = (tx, y);
            curcoord += 1;
            if self.get(tx, y) == value {
                return Err(format!("Value {} is already in the row {}", value, y));
            }
        }
        // verify by sector
        let sx = x / 3;
        let sy = y / 3;
        for tx in sx * 3..sx*3+3 {
            for ty in sy * 3..sy*3+3 {
                coords[curcoord] = (tx, ty);
                curcoord += 1;
                if self.get(tx, ty) == value {
                    return Err(format!(
                        "Value {} is already in sector ({}, {})", value, sx, sy));
                }
            }
        }
        // Finally, we can set the value
        self.data[x][y] = value;

        // Remove option from relevant cells
        for (x, y) in &coords {
            self.remove_option(*x, *y, value);
        }
        Ok(())
    }

    fn remove_option(&mut self, x: usize, y: usize, value: u8) {
        let optidx = (value - 1) as usize;
        if self.options[x][y][optidx] != 0 {
            self.optcounts[x][y] -= 1;
            self.options[x][y][optidx] = 0;
        }
    }

    fn count_options(&self, x: usize, y: usize) -> u8 {
        return self.optcounts[x][y];
    }

    fn iter_empty_coords(&self) -> EmptyCells {
        EmptyCells::new(self)
    }

    fn get_minimum_options_coord(&self) -> (usize, usize) {
        let mut minimum = (0, 0);
        let mut mincount = 9 as u8;
        for (x, y) in self.iter_empty_coords() {
            let count = self.count_options(x, y);
            if count == 2 {
                // We are not going to find anything better than this, shortcut
                return (x, y);
            }
            if  count < mincount {
                mincount = count;
                minimum = (x, y);
            }
        }
        minimum
    }

    // Problem is solved when all the cells are filled
    fn is_solved(&self) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if self.get(x, y) == 0 {
                    return false;
                }
            }
        }
        true
    }

    // Problem is solvable if all empty cells have at least one option
    fn is_solvable(&self) -> bool {
        for x in 0..9 {
            for y in 0..9 {
                if self.get(x, y) == 0 {
                    if self.count_options(x, y) == 0 {
                        return false;
                    }
                }
            }
        }
        true
    }

    fn format(&self) -> String {
        let mut out = String::new();
        for block in 0..3 {

            for line in 0..3 {
                let y = block*3 + line;

                let cell = |x|  {
                    let n = self.get(x, y);
                    if n != 0 {
                        format!("{}", n).chars().next().unwrap()
                    }
                    else {
                        ' '
                    }
                };

                out.push_str(
                    &format!("{} {} {} | {} {} {} | {} {} {}\n",
                        cell(0), cell(1), cell(2),
                        cell(3), cell(4), cell(5),
                        cell(6), cell(7), cell(8)))
            }

            if block != 2 {
                out.push_str("------+-------+------\n")
            }
        }
        out
    }
}

fn solve(mut problem: Problem) -> Result<Problem, String> {
    while !problem.is_solved() {
        let moves = get_trivial_moves(&problem);
        if moves.len() == 0 {
            // fork
            return fork(problem)
        }
        for (x, y, v) in moves {
            problem.set(x, y, v).unwrap();
            if !problem.is_solvable() {
                return Err("Cannot solve".to_owned());
            }
        }
    }
    Ok(problem)
}


fn fork(problem: Problem) -> Result<Problem, String> {
    // println!("Forking");
    let (fx, fy) = problem.get_minimum_options_coord();
    for n in 0..9 {
        let candidate = problem.options[fx][fy][n];
        if candidate == 0 {
            continue
        }
        let mut attempt = problem.clone();
        attempt.set(fx, fy, candidate).unwrap();
        match solve(attempt) {
            Err(_err) => continue,
            Ok(solved) => return Ok(solved)
        }
    }
    Err("Cannot solve".to_owned())
}

fn get_trivial_moves(problem: &Problem) -> Vec<(usize, usize, u8)> {
    let mut moves = Vec::with_capacity(9);
    for (x, y) in problem.iter_empty_coords() {
        if problem.count_options(x, y) != 1 {
            continue;
        }
        let opts = problem.options[x][y];
        for n in 0..9 {
            if opts[n] != 0 {
                moves.push((x, y, opts[n]))
            }
        }
    }
    moves
}

fn read_problem(filename: &String) -> Problem {
    let mut problem = Problem::new();
    let contents = fs::read_to_string(filename)
        .expect("Cannot open file");
    for (y, line) in contents.split("\n").enumerate() {
        for (x, snum) in line.split(" ").enumerate() {
            let num = snum.parse::<u8>().unwrap();
            if num != 0 {
                problem.set(x, y, num).unwrap();
            }
        }
        if y == 8 {
            break;
        }
    }
    problem
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let problem = read_problem(&args[1]);
    println!("Initial problem:");
    print!("{}", problem.format());

    let copied = problem.clone();
    let solved = solve(copied).unwrap();

    // Now solve it several more times for benchmark
    for _i in 0..0 {
        let copied = problem.clone();
        solve(copied).unwrap();
    }

    println!("Solved problem:");
    print!("{}", solved.format());
}
