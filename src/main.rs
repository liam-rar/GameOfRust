mod preset;
mod lib;

use crate::lib::*;
use std::io::{self, Write,};
use std::thread;
use std::time::Duration;
use crossterm::{ ExecutableCommand, terminal, cursor };

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.execute(cursor::MoveTo(0, 0))?;
    stdout.execute(cursor::Hide)?;
    let mut life_cycles: i32 = 0;

    let term = terminal::size()?;
    let (width, mut height) = term;
    height -= 1;

    let mut board = Board::new(width, height);

    let (start_x, start_y) = (width/2, height/2);
    preset::set_glider(&mut board, start_x, start_y);
    //set_blinker(&mut board, start_x, start_y);
    //set_block(&mut board, start_x, start_y);
    //set_toad(&mut board, start_x, start_y);
    //set_beacon(&mut board, start_x, start_y);
    //set_pulsar(&mut board, start_x, start_y);


    let mut new_board = Board::new(width, height);

    loop{
        display(&board, &mut stdout)?;
        step(&board, &mut new_board);
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
