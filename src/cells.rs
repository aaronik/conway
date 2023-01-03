use std::collections::HashSet;

pub struct Cells {
    living_cells: HashSet<String>,
}

impl Cells {
    pub fn new() -> Cells {
        Cells { living_cells: HashSet::new() }
    }

    pub fn birth(&mut self, i: u32, j: u32) {
        self.living_cells.insert(format_cell_key(i, j));
    }

    pub fn kill(&mut self, i: u32, j: u32) {
        self.living_cells.remove(&format_cell_key(i, j));
    }

    pub fn is_alive(&self, i: u32, j: u32) -> bool {
        self.living_cells.contains(&format_cell_key(i, j))
    }

    pub fn num_living_neighbors(&self, i: u32, j: u32) -> u32 {
        let neighbors = vec![
            format_cell_key(i - 1, j - 1),
            format_cell_key(i - 1, j),
            format_cell_key(i - 1, j + 1),
            format_cell_key(i, j - 1),
            format_cell_key(i, j + 1),
            format_cell_key(i + 1, j - 1),
            format_cell_key(i + 1, j),
            format_cell_key(i + 1, j + 1),
        ];

        let mut num_living_neighbors = 0;

        neighbors.iter().for_each(|key| {
            if self.living_cells.contains(key) {
                num_living_neighbors += 1;
            }
        });

        num_living_neighbors
    }
}

fn format_cell_key(i: u32, j: u32) -> String {
    format!("{i}-{j}")
}

