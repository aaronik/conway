pub struct Snapshot {
    previous: String,
    current: String,
}

impl Snapshot {
    pub fn new() -> Snapshot {
        Snapshot {
            previous: String::from(""),
            current: String::from(""),
        }
    }

    pub fn add(&mut self, i: u32, j: u32) {
        self.current.push_str(&format!("{i}-{j}|"));
    }

    /// Is the current build up snapshot the same as the previous one, the one that most recently
    /// got cycled?
    pub fn is_same(&self) -> bool {
        self.current == self.previous
    }

    /// Move the current snapshot to the previous slot and reset current for another round
    pub fn cycle(&mut self) {
        self.previous.clear();
        self.previous
            .push_str(&self.current.drain(..self.current.len()).collect::<String>()[..]);
        self.current.clear();
    }
}


// This needs to:
// * Keep a list of every state in the history of an evolution
// * Be able to compare those states and return equal for two equivalent states
// * However there's no guarantee from our Cells grid the order in which living cells
// will be returned, so we need to be order agnostic
// * Be fast

// use std::collections::{HashSet, LinkedList};

// pub struct Snapshot {
//     mem1: Vec<Vec<(u32, u32)>>, // Can't guarantee order, would have to sort each inner vec
//     mem2: Vec<HashSet<(u32, u32)>>, // Can probably guarantee equality, but would be too big?
//     mem3: LinkedList<HashSet<(u32, u32)>>, // Might save speed by non requiring contiguous memory.
//                                            // Help doc says vec is better
//     mem4: Vec<String>, // Make a string out of each coord. Trivial comparison but strings are slow.
//     mem5: Vec<u64>,    // This would be ideal, but how to get a unique number for each coord?
//                        // * Assign a prime for each coord. Then it'd just be about multiplying them
//                        // together. But then I'd need ~22k primes. And how to store numbers so big
//                        // like multiplying the first 10k primes together?
//     mem6: HashSet<String>, // Where String comes from the serialization of a BTreeSet. Yes, each
//                            // cell in the grid can be represented by a number: SIZE*i + j. Thus we
//                            // can have ordering, and a BTreeSet. That set can be serialized
//                            // deterministcally into a string, which can be stored in a hashset.
//                            // Along side this hash set can be a vector with the strings, for
//                            // ordering. So at each iteration of the grid, we can check in O(1)
//                            // if this has been a state that we've seen yet, and then if so we can
//                            // see how far back it happened, and that's our fitness.
// }

// impl Snapshot {
//     pub fn new() -> Snapshot {
//         Snapshot {

//         }
//     }

//     pub fn add(&mut self, i: u32, j: u32) {
//         self.current.push_str(&format!("{i}-{j}|"));
//     }

//     /// Is the current build up snapshot the same as the previous one, the one that most recently
//     /// got cycled?
//     pub fn is_same(&self) -> bool {
//         self.current == self.previous
//     }

//     /// Move the current snapshot to the previous slot and reset current for another round
//     pub fn cycle(&mut self) {
//         self.previous.clear();
//         self.previous
//             .push_str(&self.current.drain(..self.current.len()).collect::<String>()[..]);
//         self.current.clear();
//     }
// }
