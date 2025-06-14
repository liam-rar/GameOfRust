use std::io::{self, Write};
use crossterm::{
    ExecutableCommand, QueueableCommand,
    terminal, cursor, style::{self, Stylize}
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    //display dimensions
    let yd = 30;
    let xd = 100;

    // Define starting positions for two cells
    let cell1 = (15, 15); // (x, y) coordinates for first cell
    let cell2 = (16, 15); // (x, y) coordinates for second cell

    for y in 0..yd {
        for x in 0..xd {
            let symbol = if (x, y) == cell1 || (x, y) == cell2 {
                "#"
            } else {
                "."
            };
            
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(style::PrintStyledContent(symbol.white()))?;
        }
    }
    stdout.flush()?;
    Ok(())
}
