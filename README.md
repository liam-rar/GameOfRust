### Cell Calculation Theory:
- Position Calculation
    - Each cell has an (x,y) coordinate in the grid
    - When using a single Vec<bool>, the position is calculated as: index = y * width + x
    - This converts 2D coordinates into a 1D array index
    - The width is crucial as it tells us how many cells to skip to move down a row
- Neighbor Calculation
    - Each cell has 8 neighbors (except edge cells)
    - Neighbors are in these relative positions:
    - Apply to main.rs
    - To find neighbors, you add these offsets to the current cell's coordinates
- Edge Handling
    - You have several options for handling edges:
    - Toroidal (wrapping): Cells on the right edge connect to the left edge
    - Dead edges: Consider cells outside the grid as dead
    - Infinite grid: Allow the grid to grow as needed
    - Periodic boundaries: Similar to toroidal but with different rules
- Neighbor Counting Logic
    - For each cell, you need to:
    - Check all 8 surrounding positions
    - Count how many are alive
    - Apply the Game of Life rules based on this count
- Optimization Considerations
    - You can optimize by:
        - Only checking cells that are alive or have live neighbors
        - Using bit operations for faster counting
        - Caching neighbor counts
        - Using parallel processing for large grids

# Game Logic
- Any live cell with fewer than 2 live neighbors dies (underpopulation)
- Any live cell with 2 or 3 live neighbors lives
- Any live cell with more than 3 live neighbors dies (overpopulation)
- Any dead cell with exactly 3 live neighbors becomes alive (reproduction)

# Game Loop
- Add a main game loop that:
    - Displays the current state (you already have the display logic)
    - Computes the next generation
    - Updates the display
    - Adds a small delay between generations
