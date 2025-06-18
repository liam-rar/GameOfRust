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
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(cursor::Hide)?;
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

    let (start_x, start_y) = (width/2, height/2);
    set_glider(&mut board, start_x, start_y);
    //set_blinker(&mut board, start_x, start_y);
    //set_block(&mut board, start_x, start_y);
    //set_toad(&mut board, start_x, start_y);
    //set_beacon(&mut board, start_x, start_y);
    //set_pulsar(&mut board, start_x, start_y);

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
        stdout.execute(cursor::MoveTo(0, 0))?;
        if life_cycles == 101 { break }
    }

    stdout.flush()?;
    stdout.execute(cursor::Show)?;
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

fn set_blinker(board: &mut Board, start_x: u16, start_y: u16) {
    let positions = [
        (start_x, start_y),
        (start_x, start_y + 1),
        (start_x, start_y + 2),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}

fn set_block(board: &mut Board, start_x: u16, start_y: u16) {
    let positions = [
        (start_x, start_y),
        (start_x + 1, start_y),
        (start_x, start_y + 1),
        (start_x + 1, start_y + 1),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}

fn set_toad(board: &mut Board, start_x: u16, start_y: u16) {
    let positions = [
        (start_x + 1, start_y),
        (start_x + 2, start_y),
        (start_x + 3, start_y),
        (start_x, start_y + 1),
        (start_x + 1, start_y + 1),
        (start_x + 2, start_y + 1),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}

fn set_beacon(board: &mut Board, start_x: u16, start_y: u16) {
    let positions = [
        (start_x, start_y),
        (start_x + 1, start_y),
        (start_x, start_y + 1),
        (start_x + 1, start_y + 1),
        (start_x + 2, start_y + 2),
        (start_x + 3, start_y + 2),
        (start_x + 2, start_y + 3),
        (start_x + 3, start_y + 3),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}

fn set_pulsar(board: &mut Board, start_x: u16, start_y: u16) {
    let positions = [
        // Top left
        (start_x + 2, start_y), (start_x + 3, start_y), (start_x + 4, start_y),
        (start_x + 8, start_y), (start_x + 9, start_y), (start_x + 10, start_y),
        // Top right
        (start_x + 12, start_y + 2), (start_x + 12, start_y + 3), (start_x + 12, start_y + 4),
        (start_x + 12, start_y + 8), (start_x + 12, start_y + 9), (start_x + 12, start_y + 10),
        // Bottom left
        (start_x, start_y + 2), (start_x, start_y + 3), (start_x, start_y + 4),
        (start_x, start_y + 8), (start_x, start_y + 9), (start_x, start_y + 10),
        // Bottom right
        (start_x + 2, start_y + 12), (start_x + 3, start_y + 12), (start_x + 4, start_y + 12),
        (start_x + 8, start_y + 12), (start_x + 9, start_y + 12), (start_x + 10, start_y + 12),
        // Center arms
        (start_x + 6, start_y + 2), (start_x + 6, start_y + 3), (start_x + 6, start_y + 4),
        (start_x + 6, start_y + 8), (start_x + 6, start_y + 9), (start_x + 6, start_y + 10),
        (start_x + 2, start_y + 6), (start_x + 3, start_y + 6), (start_x + 4, start_y + 6),
        (start_x + 8, start_y + 6), (start_x + 9, start_y + 6), (start_x + 10, start_y + 6),
    ];

    for (x, y) in positions {
        if x < board.width && y < board.height {
            let index = y * board.width + x;
            board.cells[index as usize] = true;
        }
    }
}



















