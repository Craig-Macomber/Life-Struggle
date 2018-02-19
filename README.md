# Life-Struggle
1 vs 1 competitive version of Conway's Game of Life.
Each player provides a tile design of the same square dimensions.
The world is an infinite plane of tiles, split along x=0, with each side covered with one of the players tiles,
tiled endlessly.

After some fixed number of generations, a score is computed:
- 1 point added for each tile of enemy territory converted into your tile
- 1 point deducted for each tile of your territory disrupted

A tile is determine to match yours if its current state is the same as your tile would be on that generation
if it were in a world of just itself tiled endlessly.

The eventual focus is more in the direction of using AI to generate effective tiles than as a game for humans.
The project is largely an exercise in learning rust.
The implementation does not use advanced Life simulation algorithms (like hash life), though it does basic
parallelization.
