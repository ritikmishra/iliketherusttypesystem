//! implementation of conway's game of life inside the type system

#[cfg(test)]
#[allow(unused)]
mod working_regular_impl {
    use std::{collections::HashMap, convert::TryInto};

    type Cell = (i32, i32);

    fn iterate(active_cells: &mut Vec<Cell>) {
        fn get_cell_neighbors(cell: Cell) -> Vec<Cell> {
            let mut ret = Vec::new();

            let (x, y) = cell;
            for i in -1..=1 {
                for j in -1..=1 {
                    if i != 0 || j != 0 {
                        ret.push((x + i, y + j));
                    }
                }
            }

            ret
        }

        let mut neighbor_counts: HashMap<Cell, i32> = HashMap::new();
        for cell in active_cells.iter().copied() {
            for neighbor in get_cell_neighbors(cell) {
                *neighbor_counts.entry(neighbor).or_insert(0) += 1
            }
        }

        *active_cells = neighbor_counts
            .into_iter()
            .filter(|(cell, count)| *count == 3 || (active_cells.contains(cell) && *count == 2))
            .map(|(cell, _)| cell)
            .collect();
    }

    fn print_board(board: &Vec<Cell>) -> Option<()> {
        let smallest_x = board.iter().copied().map(|(x, y)| x).min()?;
        let largest_x = board.iter().copied().map(|(x, y)| x).max()?;
        let smallest_y = board.iter().copied().map(|(x, y)| y).min()?;
        let largest_y = board.iter().copied().map(|(x, y)| y).max()?;

        for i in smallest_y..=largest_y {
            for j in smallest_x..=largest_x {
                if board.contains(&(j, i)) {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            print!("\n");
        }

        Some(())
    }
}

