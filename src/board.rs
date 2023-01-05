pub struct Board {
    pub id: Option<u32>,
    pub size: u32,
    pub cells: Vec<(u32, u32)>, // starting cells
    pub iterations: usize, // Num unique iterations the board had after playing out
    pub period: Option<usize>, // Period if a loop was encountered, otherwise None
}
