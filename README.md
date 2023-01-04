# Conway's game of life

This is a rust implementation of Conway's game of life. It has a few notable features:

* Terminal output -- This program prints output to a terminal using brail
  characters, which can be many per character space. In this way, it allows for a large output space in
  a small terminal.

* Colorful output -- The output is colorful! The colors go from cool to warm based on the age of the cell.
  The youngest cells are black, and the oldest are red. Note that every character must have the same color,
  so there will be inaccuracies when there are multiple cells in one character that need different colors.
