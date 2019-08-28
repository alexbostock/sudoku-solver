mod sudoku {
    #[derive(Clone)]
    enum Cell {
        Value(i8),
        PossibleValues(std::collections::HashSet<i8>),
    }

    #[derive(Clone)]
    pub struct Puzzle {
        // A puzzle is n x n (so standard sudoku puzzles have n = 9)
        n: i8,
        grid: Vec<Vec<Cell>>,
    }

    pub fn brute_force(p: Puzzle) -> Option<Puzzle> {
        let r = Puzzle {
            n: p.n,
            grid: Vec::new(),
        };

        return try_combinations(&p, &r);
    }

    fn try_combinations(p: &Puzzle, r: &Puzzle) -> Option<Puzzle> {
        let mut s = r.clone();

        for (i, row) in p.grid.iter().enumerate() {
            s.grid.push(Vec::new());
            for (j, cell) in row.iter().enumerate() {
                match cell {
                    Cell::Value(v) => s.grid[i].push(Cell::Value(*v)),
                    Cell::PossibleValues(vs) => {
                        s.grid[i].push(Cell::Value(0));

                        for (num, v) in vs.iter().enumerate() {
                            s.grid[i][j] = Cell::Value(*v);
                            if let Some(puzzle) = try_combinations(p, &s) {
                                s = puzzle;
                                break;
                            } else if num == vs.len() - 1 {
                                return None;
                            }
                        }
                    }
                }
            }
        }

        return Some(s);
    }
}

fn main() {
    println!("Hello, world!");
}
