use std::io::{self, Write,};
use std::thread;
use std::time::Duration;
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

struct Board {
    cells: Vec<bool>,
    width: u16,
    height: u16,
}

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;
    let mut life_cycles: i32 = 0;

    let term = terminal::size()?;
    let (width, mut height) = term;
    //smaller window size so I can write info at the bottom
    height = height - 1;
    let size = width * height;

    let mut board = Board {
        cells:vec![false; size as usize],
        width: width,
        height: height,
    };

    //find middle of board
    let startCell = (width * height)/2;

    let (start_x, start_y) = (width/2, height/2);
    set_glider(&mut board, start_x, start_y);

    //starting pattern
//  board.cells[startCell as usize + 1] = true;
//  board.cells[startCell as usize + 2] = true;
//  board.cells[startCell as usize] = true;

    let mut new_board = Board {
        cells:vec![false; size as usize],
        width: width,
        height: height,
    };

    loop{
        for y in 0..board.height {
            for x in 0..board.width {
                let index = y * board.width + x;
                let neighbor_count = neighbors(&board, x, y);

                //rules
                let current_cell = board.cells[index as usize];
                let next_cell = match (current_cell, neighbor_count) {
                    (true, 2) | (true, 3) => true, //survive
                    (false, 3) => true, //reproduce
                    _ => false //dies
                };

                new_board.cells[index as usize] = next_cell;

                //set display for current generation
                let symbol = if board.cells[index as usize] {
                    "#"
                } else {
                    "."
                };
                stdout
                    .queue(cursor::MoveTo(x as u16, y as u16))?
                    .queue(style::PrintStyledContent(symbol.white()))?;
            }
        }

        board.cells.copy_from_slice(&new_board.cells);

        println!("life cycles: {}, window size = x: {} y: {}", life_cycles, width, height);
        thread::sleep(Duration::from_millis(500));
        life_cycles = life_cycles + 1;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        if life_cycles == 101 { break }
    }

    stdout.flush()?;
    Ok(())
}

fn neighbors (board:&Board, x:u16, y:u16) -> u8 {
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

fn set_glider(board: &mut Board, start_x: u16, start_y: u16) {
    // Set a glider pattern starting at (start_x, start_y)
    let positions = [
        (start_x, start_y),
        (start_x + 1, start_y),
        (start_x + 2, start_y),
        (start_x + 2, start_y - 1),
        (start_x + 1, start_y - 2),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}





























