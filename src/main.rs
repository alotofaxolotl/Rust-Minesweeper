use rand::{Rng, rngs::ThreadRng};

#[derive(Debug)]
struct Cell {
    clear: bool,
    flagged: bool,
    mine: bool
}

fn build_cell(mine_chance: &f32, rng: &mut ThreadRng) -> Cell {
    Cell {
        clear: false,
        flagged: false,
        mine: (rng.gen::<f32>() < *mine_chance) as bool
    }
}

fn build_field(size: usize, mine_chance: f32) -> Vec<Vec<Cell>> {
    let mut rng = rand::thread_rng();
    let mut field: Vec<Vec<Cell>> = Vec::new();
    for _ in 0..size {
        field.push(vec![build_cell(&mine_chance, &mut rng)]);
    }
    field
}

fn main() {
    let mut field = build_field(10, 0.1);
}
