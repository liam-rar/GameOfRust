use std::io::Stdout;
use crossterm::{cursor, style::{self, Stylize}, QueueableCommand};

pub struct Board {
    pub cells: Vec<bool>,
    pub width: u16,
    pub height: u16,
}

impl Board {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            cells: vec![false; (width * height) as usize],
            width,
            height,
        }
    }
}

pub fn neighbors (board:&Board, x:u16, y:u16) -> u8 {
    let mut live = 0;

    // Check all 8 neighbors, but only if they're within bounds
    let neighbors = [
        (x.checked_sub(1), y.checked_sub(1)),     // top-left
        (Some(x), y.checked_sub(1)),              // top
        (x.checked_add(1), y.checked_sub(1)),     // top-right
        (x.checked_sub(1), Some(y)),              // left
        (x.checked_add(1), Some(y)),              // right
        (x.checked_sub(1), y.checked_add(1)),     // bottom-left
        (Some(x), y.checked_add(1)),              // bottom
        (x.checked_add(1), y.checked_add(1)),     // bottom-right
    ];

    for (nx, ny) in neighbors {
        if let (Some(nx), Some(ny)) = (nx, ny) {
            if nx < board.width && ny < board.height {
                let neighbor_index = ny * board.width + nx;
                if board.cells[neighbor_index as usize] {
                    live += 1;
                }
            }
        }
    }
    live
}

pub fn step(current: &Board, next: &mut Board) {
        for y in 0..current.height {
            for x in 0..current.width {
                let index = y * current.width + x;
                let neighbor_count = neighbors(&current, x, y);

                //rules
                let current_cell = current.cells[index as usize];
                let next_cell = match (current_cell, neighbor_count) {
                    (true, 2) | (true, 3) => true, //survive
                    (false, 3) => true, //reproduce
                    _ => false //dies
                };

            next.cells[index as usize] = next_cell;
        }
    }
}

pub fn display(board: &Board, stdout: &mut Stdout) -> std::io::Result<()> {
    for y in 0..board.height {
        for x in 0..board.width {
            let index = y * board.width + x;
            let symbol = if board.cells[index as usize] { "#" } else { "." };
            stdout
            .queue(cursor::MoveTo(x as u16, y as u16))?
            .queue(style::PrintStyledContent(symbol.white()))?;
        }
    }
    Ok(())
}
