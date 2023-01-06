use std::collections::{HashMap, HashSet};

/// An abstraction of binary entries on a 2d grid
/// The responsibilities here are to keep track of the cells in the grid and supply helper methods
/// to manage those cells
pub struct Cells {
    extent_cells: HashMap<(u32, u32), usize>,
    uncommitted_cells: HashMap<(u32, u32), usize>,
    size: u32,
}

impl Cells {
    pub fn new(size: u32) -> Cells {
        Cells {
            extent_cells: HashMap::new(),
            uncommitted_cells: HashMap::new(),
            size,
        }
    }

    /// Mark this cell as alive (present in our internal hash set)
    /// If this cell already was alive, its age gets incremented
    pub fn birth(&mut self, i: u32, j: u32) {
        let prev_count = self.extent_cells.get(&(i, j)).unwrap_or(&0);
        self.uncommitted_cells.insert((i, j), prev_count + 1);
    }

    /// A convenience method to mark multiple cells as alive at the same time, like:
    /// birth_multiple(&[
    ///     (5, 5),
    ///     (6,6),
    /// ])
    /// This assumes this is used just for initial set up, so all ages are set to 1.
    pub fn birth_multiple(&mut self, coords: &[(u32, u32)]) {
        coords.iter().for_each(|coord| {
            self.uncommitted_cells.insert((coord.0, coord.1), 1);
        })
    }

    /// Mark this cell as not alive (no longer present in our internal hash)
    pub fn kill(&mut self, i: u32, j: u32) {
        self.uncommitted_cells.remove(&(i, j));
    }

    /// Is the given coord a living cell (as opposed to an empty or dead one)?
    pub fn is_alive(&self, i: u32, j: u32) -> bool {
        self.extent_cells.contains_key(&(i, j))
    }

    /// Returns the age of the cell, or 0 if it's not alive
    pub fn get_age(&self, i: u32, j: u32) -> usize {
        *self.extent_cells.get(&(i, j)).unwrap_or(&(0 as usize))
    }

    /// How many living cells are there?
    pub fn num_living_cells(&self) -> usize {
        self.extent_cells.len()
    }

    /// How many living neighbors are there of the given coord?
    pub fn num_living_neighbors(&self, i: u32, j: u32) -> usize {
        let neighbors = self.neighbors(i, j);

        let mut num_living_neighbors = 0;

        neighbors.iter().for_each(|key| {
            if self.extent_cells.contains_key(key) {
                num_living_neighbors += 1;
            }
        });

        num_living_neighbors
    }

    /// Get a list of just the living cells
    pub fn living_cells(&self) -> Vec<(u32, u32)> {
        // TODO Is it dangerous to give away dereferenced locations like this?
        // Whoever gets it, if they modify it, aren't they changing our data?
        self.extent_cells.keys().map(|key| *key).collect()
    }

    /// Get a list of all the living cells and their neighbors, living or not
    pub fn living_cells_and_neighbors(&self) -> HashSet<(u32, u32)> {
        // Start a new hashset (for uniqueness)
        let mut res = HashSet::new();

        // Iterate over all our living fellas
        self.extent_cells.iter().for_each(|(coord, ..)| {
            // Add the living fella
            res.insert(*coord);

            // Add all its neighbors
            self.neighbors(coord.0, coord.1).iter().for_each(|coord| {
                res.insert(*coord);
            });
        });

        res
    }

    /// All the neighbors of a given coord, as tuples, wrapped around the size of the board
    pub fn neighbors(&self, i: u32, j: u32) -> Vec<(u32, u32)> {
        let plus = |n: u32| -> u32 {
            if n == self.size {
                0
            } else {
                n + 1
            }
        };

        let minus = |n: u32| -> u32 {
            if n == 0 {
                self.size
            } else {
                n - 1
            }
        };

        vec![
            (minus(i), minus(j)),
            (minus(i), j),
            (minus(i), plus(j)),
            (i, minus(j)),
            (i, plus(j)),
            (plus(i), minus(j)),
            (plus(i), j),
            (plus(i), plus(j)),
        ]
    }

    /// When you perform mutable operations that add/remove cells from the grid,
    /// they need to be stored in a secondary location. This is b/c if you add a cell
    /// to say grid 1, then check grid 2 for the rules, it'll include grid 1. This is
    /// incorrect behavior. To boot, since we use Hashes, iterating through the keys
    /// comes in an undefined order, which results in nondeterministic behavior on the board,
    /// which should be deterministic.
    ///
    /// This function takes all the adds/removes that've happened and commits them to the grid.
    /// When you call a read function, like num_living_neighbors, it reads from the grid.
    /// When you call a writing function, like birth/klll, it adds to a temporary grid.
    /// Commit flushes the grid and copies the temporary grid to the grid.
    pub fn commit(&mut self) {
        self.extent_cells.clear();
        self.uncommitted_cells.drain().for_each(|coord| {
            self.extent_cells.insert(coord.0, coord.1);
        });
    }
}
