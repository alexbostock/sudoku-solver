mod sudoku {
    enum PuzzleSize {
        Four,
        Six,
        Nine,
        Twelve,
        Sixteen,
        Twenty,
        TwentyFive,
    }

    enum Cell {
        Value(i8),
        PossibleValues(std::collections::HashSet<i8>),
    }

    struct Puzzle {
        size: PuzzleSize,
        grid: Vec<Vec<Cell>>,
    }
}

fn main() {
    println!("Hello, world!");
}
