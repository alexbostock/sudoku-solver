mod sudoku {
    #[derive(Clone)]
    enum Cell {
        Value(u8),
        PossibleValues(std::collections::HashSet<u8>),
    }

    #[derive(Clone)]
    pub struct Puzzle {
        // A puzzle is n x n (so standard sudoku puzzles have n = 9)
        n: u8,
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
        if !is_valid(r) {
            return None;
        }

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

    fn is_valid(p: &Puzzle) -> bool {
        // TODO: learn custom iterators etc. to make this function more readable

        // Check each row
        let ok = p.grid.iter().fold(true, |acc, row| {
            acc && !row
                .iter()
                .fold(
                    (1..=p.n.pow(2)).clone().collect(),
                    |mut digits: std::collections::HashSet<u8>, x: &Cell| match x {
                        Cell::Value(v) => {
                            digits.remove(&v);
                            digits
                        }
                        Cell::PossibleValues(vs) => digits.difference(&vs).cloned().collect(),
                    },
                )
                .is_empty()
        }) &&

        // Column
        (1..=p.n.pow(2)).fold(true, |acc, col| {
            acc && !p
                .grid
                .iter()
                .fold((1..=p.n.pow(2)).clone().collect(), |mut digits: std::collections::HashSet<u8>, x| match &x[usize::from(col)] {
                    Cell::Value(v) => {
                        digits.remove(&v);
                        digits
                    }
                    Cell::PossibleValues(vs) => digits.difference(&vs).cloned().collect(),
                })
                .is_empty()
        });

        if !ok {
            return false;
        }

        // Box
        let n = p.n as usize;

        for box_row in 0..n {
            for box_col in 0..n {
                let mut digits: std::collections::HashSet<u8> = (1..=p.n.pow(2)).clone().collect();

                for i in 0..n {
                    for j in 0..n {
                        match &p.grid[box_row * n + i][box_col * n + j] {
                            Cell::Value(v) => {
                                digits.remove(&v);
                            }
                            Cell::PossibleValues(vs) => {
                                digits = digits.difference(&vs).cloned().collect();
                            }
                        };
                    }
                }

                if !digits.is_empty() {
                    return false;
                }
            }
        }

        return true;
    }
}

fn main() {
    println!("Hello, world!");
}
