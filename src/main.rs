use std::io::{self, Write};
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

    //get terminal size (returns tuple)
    let term = terminal::size()?;

    let (width, mut height) = term;
    //smaller window size so I can write info at the bottom
    height = height - 1;
    let size = width * height;

    let board = Board {
        cells:vec![false; size as usize],
        width: width,
        height: height,
    };

    // Define starting positions for two cells
    let cell1 = (15, 15); // (x, y) coordinates for first cell
    let cell2 = (16, 15); // (x, y) coordinates for second cell

    for y in 0..board.height {
        for x in 0..board.width {
            let symbol = if (x, y) == cell1 || (x, y) == cell2 {
                "#"
            } else {
                "."
            };

            stdout
                .queue(cursor::MoveTo(x as u16, y as u16))?
                .queue(style::PrintStyledContent(symbol.white()))?;
        }
    }

    println!("window size = x: {} y: {}", width, height);
    stdout.flush()?;
    Ok(())
}
