use std::collections::HashSet;

/// An abstraction of binary entries on a 2d grid
pub struct Cells {
    living_cells: HashSet<(u32, u32)>,
}

impl Cells {
    pub fn new() -> Cells {
        Cells {
            living_cells: HashSet::new(),
        }
    }

    /// Mark this cell as alive (present in our internal hash set)
    pub fn birth(&mut self, i: u32, j: u32) {
        self.living_cells.insert((i, j));
    }

    /// A convenience method to mark multiple cells as alive at the same time, like:
    /// birth_multiple(&[
    ///     (5, 5),
    ///     (6,6),
    /// ])
    pub fn birth_multiple(&mut self, coords: &[(u32, u32)]) {
        coords.iter().for_each(|coord| {
            self.living_cells
                .insert((coord.0, coord.1));
        })
    }

    /// Mark this cell as not alive (no longer present in our internal hash set)
    pub fn kill(&mut self, i: u32, j: u32) {
        self.living_cells.remove(&(i, j));
    }

    /// Is the given coord a living cell (as opposed to an empty or dead one)?
    pub fn is_alive(&self, i: u32, j: u32) -> bool {
        self.living_cells.contains(&(i, j))
    }

    /// How many living neighbors are there of the given coord?
    pub fn num_living_neighbors(&self, i: u32, j: u32) -> u32 {
        let neighbors = self.neighbors(i, j);

        let mut num_living_neighbors = 0;

        neighbors.iter().for_each(|key| {
            if self.living_cells.contains(key) {
                num_living_neighbors += 1;
            }
        });

        num_living_neighbors
    }

    // /// Get a list of all the living cells
    // pub fn living_cells(&self) -> Vec<(u32, u32)> {
    //     self.living_cells.iter().collect()
    // }

    /// All the neighbors of a given coord, as tuples
    pub fn neighbors(&self, i: u32, j: u32) -> Vec<(u32, u32)> {
        vec![
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, j + 1),
            (i, j - 1),
            (i, j + 1),
            (i + 1, j - 1),
            (i + 1, j),
            (i + 1, j + 1),
        ]
    }
}
