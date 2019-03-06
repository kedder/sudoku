use std::env;
use std::fs;

#[derive(Debug, Clone)]
struct Problem {
    data: [[u8; 9]; 9],
    options: [[[u8; 9]; 9]; 9]
}

#[derive(Debug)]
struct Coord {
    x: usize,
    y: usize
}

impl Problem {
    fn new() -> Problem {
        Problem {
            data: [[0; 9]; 9],
            options: [[[1, 2, 3, 4, 5, 6, 7, 8, 9]; 9]; 9]
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
        self.options[x][y][(value - 1) as usize] = 0;
    }

    fn count_options(&self, x: usize, y: usize) -> usize {
        let opts = self.options[x][y];
        let mut cnt: usize = 0;
        for i in 0..9 {
            if opts[i] != 0 {
                cnt += 1;
            }
        }
        cnt
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
    let empties = get_empty_coords(&problem);
    let first = &empties[0];
    for n in 0..9 {
        let candidate = problem.options[first.x][first.y][n];
        if candidate == 0 {
            continue
        }
        let mut attempt = problem.clone();
        attempt.set(first.x, first.y, candidate).unwrap();
        match solve(attempt) {
            Err(_err) => continue,
            Ok(solved) => return Ok(solved)
        }
    }
    Err("Cannot solve".to_owned())
}

fn get_trivial_moves(problem: &Problem) -> Vec<(usize, usize, u8)> {
    let mut moves = Vec::new();
    for coord in get_empty_coords(problem) {
        if problem.count_options(coord.x, coord.y) != 1 {
            continue;
        }
        let opts = problem.options[coord.x][coord.y];
        for n in 0..9 {
            if opts[n] != 0 {
                moves.push((coord.x, coord.y, opts[n]))
            }
        }
    }
    moves
}

fn get_empty_coords(problem: &Problem) -> Vec<Coord> {
    let mut empties = Vec::new();
    for x in 0..9 {
        for y in 0..9 {
            if problem.get(x, y) == 0 {
                empties.push(Coord{x, y});
            }
        }
    }
    empties
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
    let solved = solve(problem).unwrap();
    println!("Solved problem:");
    print!("{}", solved.format());
}
