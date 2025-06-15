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


    let mut lifeCycles: i32 = 0;

    loop{
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

        for y in 0..board.height {
            for x in 0..board.width {
                let index = y * board.width + x;

                //living cells
                if index == 190 {
                    board.cells[index as usize] = true
                }

                //set display
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
        println!("life cycles: {}, window size = x: {} y: {}", lifeCycles, width, height);
        thread::sleep(Duration::from_millis(1000));
        lifeCycles = lifeCycles + 1;
        stdout.execute(terminal::Clear(terminal::ClearType::All))?;
        if lifeCycles == 11 { break }
    }

    stdout.flush()?;
    Ok(())
}
