pub mod Board {

    /// Before it's solved and before it's saved
    pub struct Unsolved {
        pub size: u32,
        pub cells: Vec<(u32, u32)>,
    }

    /// After it's solved but still before it's saved
    pub struct Solved {
        pub size: u32,
        pub cells: Vec<(u32, u32)>,
        pub iterations: usize,
        pub period: Option<usize>,
    }

    /// From the DB, it's been solved and saved
    pub struct Saved {
        pub id: i64,
        pub size: u32,
        pub cells: Vec<(u32, u32)>,
        pub iterations: usize,
        pub period: Option<usize>,
    }

    /// For one method in evolve that needs to take one or the other
    pub enum Measurable<'a> {
        Solved(&'a Solved),
        Saved(&'a Saved)
    }
}
