// This needs to:
// * Keep a list of every state in the history of an evolution
// * Be able to compare those states and return equal for two equivalent states
// * However there's no guarantee from our Cells grid the order in which living cells
// will be returned, so we need to be order agnostic
// * Be fast
//
// TODO What're we doin here, it's actually pretty interesting and should be moved over
// to a readme showing off all the mathiness.
// Where String comes from the serialization of a BTreeSet. Yes, each
// cell in the grid can be represented by a number: SIZE*i + j. Thus we
// can have ordering, and a BTreeSet. That set can be serialized
// deterministcally into a string, which can be stored in a hashset.
// Along side this hash set can be a vector with the strings, for
// ordering. So at each iteration of the grid, we can check in O(1)
// if this has been a state that we've seen yet, and then if so we can
// see how far back it happened, and that's our fitness.

use std::collections::{BTreeSet, HashSet};

pub struct Snapshot {
    grids_vec: Vec<String>,
    grids_set: HashSet<String>,
    current_iteration_cells: BTreeSet<u32>,
    size: u32,
    has_repeat: bool,
}

impl Snapshot {
    pub fn new(size: u32) -> Snapshot {
        Snapshot {
            grids_vec: Vec::new(),
            grids_set: HashSet::new(),
            current_iteration_cells: BTreeSet::new(),
            size,
            has_repeat: false,
        }
    }

    /// Add a single cell to the uncommitted memory. The reason we do one cell at a time
    /// instead of all of them at once is so that we only have to go through the whole list
    /// of cells a single time per board iteration. One loop over all of them is enough :)
    pub fn add_cell(&mut self, i: u32, j: u32) {
        let cell_number = self.size * i + j;
        self.current_iteration_cells.insert(cell_number);
    }

    /// Is the current build up snapshot the same as the previous one, the one that most recently
    /// got cycled?
    pub fn has_repeat(&self) -> bool {
        self.has_repeat
    }

    // If has_repeat is true, this will return how long the repeat period is
    pub fn period(&self) -> Option<usize> {
        if !self.has_repeat {
            return None;
        };

        let most_recent = self.grids_vec.last().unwrap();

        // Go through our vector of states, starting from the most recent, and find how many back
        // we have to go to get to the same one.
        let period = self.grids_vec
            .iter()
            .rev()
            .enumerate()
            .position(|(index, grid)| index != 0 && grid == most_recent)
            .expect(&format!("snapshot says it has a repeat but blew up getting the period, {:#?}", self.grids_vec));

        Some(period)
    }

    /// Commit the cells that were added to memory as a single grid state.
    /// Remember we want to add each cell to this snapshot the one time we go through
    /// the list of cells. So we need this function here to be called once all of those
    /// have been gone through. And we don't know the order in which those cells will
    /// be called, so we do need to go through our own list one time here, but it's pre-sorted
    /// because it's in a binary tree so at least we don't have to sort it.
    pub fn commit_cells(&mut self) {
        let serialized = self.serialize_cells();
        self.grids_vec.push(serialized.clone()); // TODO It'd be way better if it lived in
                                                 // grids_set and was referenced in here
        let has_repeat = !self.grids_set.insert(serialized);

        if has_repeat {
            self.has_repeat = true;
        }

        self.current_iteration_cells.clear();
    }

    /// We need to turn our cells into a single string so we can store them as a key into a
    /// HashSet, which is what allows us to check and see if we have a repeat state in O(1)
    /// time, which is critical b/c these boards can get into the thousands of iterations and we
    /// can't have our check time growing with that number.
    fn serialize_cells(&self) -> String {
        let mut serialized = String::from("");

        self.current_iteration_cells.iter().for_each(|cell| {
            serialized += &cell.to_string();
            serialized += "|";
        });

        serialized
    }
}
