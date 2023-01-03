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


