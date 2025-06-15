# THIS IS A WORK IN PROGRESS, PLAN MARKED BELOW
# Data Structure
- Create a 2D grid/board to represent the game state

## Struct-based Approach
```rust
    struct Board {
       cells: Vec<bool>,
       width: usize,
       height: usize,
    }
```
- Pros:
    - Encapsulates the board logic
    - Can implement methods for common operations
    - Can add additional metadata (generation count, etc.)
    - Can implement traits like Display, Debug
- Cons:
    - Slightly more complex initial setup
    - Need to implement helper methods

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
- Implement the core rules of Conway's Game of Life:
- Any live cell with fewer than 2 live neighbors dies (underpopulation)
- Any live cell with 2 or 3 live neighbors lives
- Any live cell with more than 3 live neighbors dies (overpopulation)
- Any dead cell with exactly 3 live neighbors becomes alive (reproduction)
- Create a function to count neighbors for each cell
- Create a function to compute the next generation based on current state

# Game Loop
- Add a main game loop that:
    - Displays the current state (you already have the display logic)
    - Computes the next generation
    - Updates the display
    - Adds a small delay between generations
    - Handles user input (optional: pause, quit, etc.)

# User Interaction
- Add keyboard controls to:
    - Start/stop the simulation
    - Clear the board
    - Add/remove cells manually
    - Adjust simulation speed
    - Consider adding a simple menu system

# Optimization
- Consider using a double-buffering technique for the display
- Optimize the neighbor counting algorithm
- Consider using a more efficient data structure for sparse populations

# Additional Features
- Add different patterns (gliders, blinkers, etc.)
- Add a way to save/load patterns
- Add statistics (generation count, population count)
- Add color coding for different cell states or ages
