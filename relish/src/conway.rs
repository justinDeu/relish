#![allow(unused_imports)]
use crate::automaton::{CellularAutomaton, CellularAutomatonWorldSizeError};

use crate::dim2::{CellularAutomaton2d, Neighbors2d};

fn conway_neighbors(
    world: &Vec<Vec<bool>>,
    i: usize,
    j: usize,
    wrapping: bool,
) -> Neighbors2d<bool, 3, 3> {
    let height = world.len() as isize;
    let width = world[0].len() as isize;

    let mut neighbors = [[false; 3]; 3];

    for x in 0..3 {
        for y in 0..3 {
            let n_i = i as isize + x as isize - 1;
            let n_j = j as isize + y as isize - 1;

            neighbors[x][y] = if wrapping {
                world[((n_i + height) % height) as usize][((n_j + width) % width) as usize]
            } else if n_i >= 0 && n_i < height && n_j >= 0 && n_j < width {
                world[n_i as usize][n_j as usize]
            } else {
                false
            }
        }
    }

    Neighbors2d::Neighborhood(neighbors)
}

fn conway_evolve(neighbors: [[bool; 3]; 3]) -> bool {
    let live: u8 = neighbors
        .iter()
        .flat_map(|row| row.iter())
        .enumerate()
        .filter(|(idx, _)| *idx != 4) // Skip the center element (1,1)
        .filter(|(_, &cell)| cell)
        .count() as u8;

    (neighbors[1][1] && (live == 2 || live == 3)) || (!neighbors[1][1] && live == 3)
}

#[allow(non_snake_case)]
pub fn ConwayCellularAutomaton(
    world: Vec<Vec<bool>>,
    wrapping: bool,
) -> Result<CellularAutomaton2d<bool, 3, 3>, CellularAutomatonWorldSizeError> {
    CellularAutomaton2d::<bool, 3, 3>::new(world, conway_evolve, move |world, i, j| {
        conway_neighbors(world, i, j, wrapping)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_static_block() {
        let mut world = vec![vec![false; 4]; 4];

        world[1][1] = true;
        world[1][2] = true;
        world[2][1] = true;
        world[2][2] = true;

        let next_world = world.clone();

        let mut ca = ConwayCellularAutomaton(world, false).expect("Construction failed");

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);
    }

    #[test]
    fn test_simple_static_tub_whole() {
        let mut world = vec![vec![false; 5]; 5];

        world[1][2] = true;
        world[2][1] = true;
        world[2][3] = true;
        world[3][2] = true;

        let next_world = world.clone();

        let mut ca = ConwayCellularAutomaton(world, false).expect("Construction failed");

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);
    }

    #[test]
    fn test_simple_static_tub_oob() {
        let mut world = vec![vec![false; 3]; 3];

        world[0][1] = true;
        world[1][0] = true;
        world[1][2] = true;
        world[2][1] = true;

        let next_world = world.clone();

        let mut ca = ConwayCellularAutomaton(world, false).expect("Construction failed");

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), next_world);
    }

    #[test]
    fn test_simple_blinker() {
        let mut world = vec![vec![false; 3]; 3];

        world[0][1] = true;
        world[1][1] = true;
        world[2][1] = true;

        let orig_world = world.clone();

        let mut ca = ConwayCellularAutomaton(world, false).expect("Construction failed");

        let mut next_world = vec![vec![false; 3]; 3];

        next_world[1][0] = true;
        next_world[1][1] = true;
        next_world[1][2] = true;

        ca.step();
        assert_eq!(ca.world(), next_world);

        ca.step();
        assert_eq!(ca.world(), orig_world);

        ca.step();
        assert_eq!(ca.world(), next_world);
    }

    #[test]
    fn test_wrapping_glider() {
        let mut world = vec![vec![false; 10]; 10];

        world[2][3] = true;
        world[2][6] = true;
        world[3][2] = true;
        world[4][2] = true;
        world[4][6] = true;
        world[5][2] = true;
        world[5][3] = true;
        world[5][4] = true;
        world[5][5] = true;

        let orig_world = world.clone();

        let mut ca = ConwayCellularAutomaton(world, false).expect("Construction failed");

        let mut next_world = vec![vec![false; 10]; 10];

        next_world[3][2] = true;
        next_world[3][3] = true;
        next_world[4][1] = true;
        next_world[4][2] = true;
        next_world[4][4] = true;
        next_world[4][5] = true;
        next_world[5][2] = true;
        next_world[5][3] = true;
        next_world[5][4] = true;
        next_world[5][5] = true;
        next_world[6][3] = true;
        next_world[6][4] = true;

        ca.step();
        assert_eq!(ca.world(), next_world);

        next_world = vec![vec![false; 10]; 10];

        next_world[3][1] = true;
        next_world[3][2] = true;
        next_world[3][3] = true;
        next_world[3][4] = true;
        next_world[4][1] = true;
        next_world[4][5] = true;
        next_world[5][1] = true;
        next_world[6][2] = true;
        next_world[6][5] = true;

        ca.step();
        assert_eq!(ca.world(), next_world);

        next_world = vec![vec![false; 10]; 10];

        next_world[2][2] = true;
        next_world[2][3] = true;
        next_world[3][1] = true;
        next_world[3][2] = true;
        next_world[3][3] = true;
        next_world[3][4] = true;
        next_world[4][0] = true;
        next_world[4][1] = true;
        next_world[4][3] = true;
        next_world[4][4] = true;
        next_world[5][1] = true;
        next_world[5][2] = true;

        ca.step();
        assert_eq!(ca.world(), next_world);

        for _ in 0..17 {
            ca.step();
        }

        assert_eq!(ca.age(), 20);
        assert_eq!(ca.world(), orig_world);
    }
}
