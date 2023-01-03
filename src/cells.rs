use std::collections::HashSet;

pub struct Cells {
    living_cells: HashSet<String>,
}

impl Cells {
    pub fn new() -> Cells {
        Cells {
            living_cells: HashSet::new(),
        }
    }

    pub fn birth(&mut self, i: u32, j: u32) {
        self.living_cells.insert(format_cell_key(i, j));
    }

    pub fn birth_multiple(&mut self, coords: &[(u32, u32)]) {
        coords.iter().for_each(|coord| {
            self.living_cells
                .insert(format_cell_key(coord.0, coord.1));
        })
    }

    pub fn kill(&mut self, i: u32, j: u32) {
        self.living_cells.remove(&format_cell_key(i, j));
    }

    pub fn is_alive(&self, i: u32, j: u32) -> bool {
        self.living_cells.contains(&format_cell_key(i, j))
    }

    pub fn num_living_neighbors(&self, i: u32, j: u32) -> u32 {
        let neighbors = self.neighbors_by_key(i, j);

        let mut num_living_neighbors = 0;

        neighbors.iter().for_each(|key| {
            if self.living_cells.contains(key) {
                num_living_neighbors += 1;
            }
        });

        num_living_neighbors
    }

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

    fn neighbors_by_key(&self, i: u32, j: u32) -> Vec<String> {
        // TODO wouldn't it be nicer if I could just iterate over the returned map?
        self.neighbors(i, j)
            .iter()
            .map(|coord| format_cell_key(coord.0, coord.1))
            .collect::<Vec<String>>()
    }
}

fn format_cell_key(i: u32, j: u32) -> String {
    format!("{i}-{j}")
}
