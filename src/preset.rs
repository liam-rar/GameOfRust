use crate::Board;

pub fn set_glider(board: &mut Board, start_x: u16, start_y: u16) {
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

pub fn set_blinker(board: &mut Board, start_x: u16, start_y: u16) {
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

pub fn set_block(board: &mut Board, start_x: u16, start_y: u16) {
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

pub fn set_toad(board: &mut Board, start_x: u16, start_y: u16) {
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

pub fn set_beacon(board: &mut Board, start_x: u16, start_y: u16) {
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

pub fn set_pulsar(board: &mut Board, start_x: u16, start_y: u16) {
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
