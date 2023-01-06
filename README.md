# Conway's game of life

This is a rust implementation of Conway's game of life. It has a few notable features:

* **Terminal output** -- This program prints output to a terminal using brail
    characters, which can be many per character space. In this way, it allows
    for a large output space in a small terminal.

* **Colorful output** -- The output is colorful! The colors go from cool to
    warm based on the age of the cell. The youngest cells are black, and the
    oldest are red. Note that every character must have the same color, so
    there will be inaccuracies when there are multiple cells in one character
    that need different colors.

* **Evolution!** -- The program uses an evolutionary algorithm to evolve
    boards across a few variables looking for what it deems fit, then stores
    those in a local sqlite database. You can run the program in evolution mode
    or in display mode. Running it in display mode lets you see the evolved
    initial states stored in your db.

## Notable aspects of this program

### Snapshot

The snapshot's (`see src/snapshot.rs`) purpose is to keep track of every
frame of the board's iteration, and on each new frame, check to see if that
frame has already happened within the lifespan of this board's iteration. This
is to check to see if the board's gotten into an infinite loop.

It wasn't as easy as simply serializing the existing cells into a string,
because those cells are, for speed, kept track of in a `HashSet`, which, when
queried, returns cells in a nondeterministic order.

So the snapshotter uses a binary tree set (`BTreeSet`) to efficiently add each
cell to its internal memory, then a committing mechanism to prevent double
looping over the cells on the board. From there it can easily serialize the
_now ordered_ cells into a string and place that string into two different
internal memory structures: A `HashSet<String>`, which can tell us in O(1) time
if that state has been seen before, and a `Vec<String>` which allows to to
check, _if we already determine the state has been seen_, what the period is of
the loop the board is in.

By these mechanisms, the snapshotter never slows down as the number of unique
iterations of a board grows.

## Ways to improve

* Some speed gains could be had by leaning off the database a bit. Right now
  the evolver, after each board has been solved, gets the list of extent
  boards from the database in order to measure its current evolved board
  against all the rest. A more efficient means could be to keep all of the
  extent boards in memory in a single location (shared across threads), and
  work off of that, keeping it in sync with the db.

* The variables we iterate over are ATTOW a bit naive. The really cool thing to
  iterate over would be "groups of living cells", somehow finding a way to classify
  some subset of the grid as a single thing, then mutating that thing a bit
  and placing it around the board in different configurations. Maybe a thing
  is something that repeats, and its fitness can be how many iterations it repeats
  on.
