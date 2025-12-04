use itertools::{FoldWhile, Itertools};

use crate::harness::Harness;

fn get(grid: &Vec<Vec<char>>, coords: (usize, usize)) -> char {
    let (row, column) = coords;
    grid[row][column]
}

const OFFSETS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

pub fn part_1(harness: &Harness) -> String {
    let grid = harness
        .input()
        .lines()
        .map(ToString::to_string)
        .map(|line| line.as_str().chars().collect::<Vec<char>>())
        .collect_vec();

    (0..grid.len())
        .cartesian_product(0..grid[0].len())
        .filter(|&index| get(&grid, index) == '@')
        .map(|(row, column)| {
            OFFSETS
                .into_iter()
                .filter(|(offset_row, offset_column)| {
                    let new_row = row as isize + offset_row;
                    let new_column = column as isize + offset_column;

                    if new_row < 0
                        || new_column < 0
                        || new_row >= grid.len() as isize
                        || new_column >= grid[0].len() as isize
                    {
                        return false;
                    } else {
                        let new_row = new_row as usize;
                        let new_column = new_column as usize;

                        return get(&grid, (new_row, new_column)) == '@';
                    }
                })
                .count()
        })
        .filter(|&count| count < 4)
        .count()
        .to_string()
}

pub fn part_2(harness: &Harness) -> String {
    let grid = harness
        .input()
        .lines()
        .map(ToString::to_string)
        .map(|line| line.as_str().chars().collect::<Vec<char>>())
        .collect_vec();

    let grid_height = grid.len();
    let grid_width = grid[0].len();

    struct State {
        count: usize,
        grid: Vec<Vec<char>>,
    }

    std::iter::repeat(())
        .fold_while(State { count: 0, grid }, |state, _| {
            match (0..grid_height)
                .cartesian_product(0..grid_width)
                .filter(|&index| get(&state.grid, index) == '@')
                .map(|(row, column)| {
                    (
                        (row, column),
                        OFFSETS
                            .into_iter()
                            .filter(|(offset_row, offset_column)| {
                                let new_row = row as isize + offset_row;
                                let new_column = column as isize + offset_column;

                                if new_row < 0
                                    || new_column < 0
                                    || new_row >= grid_height as isize
                                    || new_column >= grid_width as isize
                                {
                                    return false;
                                } else {
                                    let new_row = new_row as usize;
                                    let new_column = new_column as usize;

                                    return get(&state.grid, (new_row, new_column)) == '@';
                                }
                            })
                            .count(),
                    )
                })
                .filter(|&(_, count)| count < 4)
                .next()
            {
                Some(((row, column), _)) => FoldWhile::Continue(State {
                    count: state.count + 1,
                    grid: state
                        .grid
                        .iter()
                        .enumerate()
                        .map(|(r, line)| {
                            line.iter()
                                .enumerate()
                                .map(|(c, &ch)| if r == row && c == column { '.' } else { ch })
                                .collect::<Vec<char>>()
                        })
                        .collect::<Vec<Vec<char>>>(),
                }),
                None => FoldWhile::Done(state),
            }
        })
        .into_inner()
        .count
        .to_string()
}
