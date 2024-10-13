use crate::automaton::{CellularAutomaton, CellularAutomatonWorldSizeError};

pub enum Neighbors2d<const S: usize, const T: usize> {
    Neighborhood([[bool; S]; T]),
    Edge,
}

pub struct CellularAutomaton2d<const S: usize, const T: usize> {
    world: Vec<Vec<bool>>,
    generation: usize,
    evolvution_fn: Box<dyn Fn([[bool; S]; T]) -> bool>,
    neighborhood_fn: Box<dyn Fn(&Vec<Vec<bool>>, usize, usize) -> Neighbors2d<S, T>>,
}

impl<const S: usize, const T: usize> CellularAutomaton2d<S, T> {
    pub fn new(
        world: Vec<Vec<bool>>,
        evolvution_fn: impl Fn([[bool; S]; T]) -> bool + 'static,
        neighborhood_fn: impl Fn(&Vec<Vec<bool>>, usize, usize) -> Neighbors2d<S, T> + 'static,
    ) -> Result<Self, CellularAutomatonWorldSizeError> {
        // TODO: check world size for functions
        // TODO: check that world is uniform across all rows
        Ok(Self {
            world,
            generation: 0,
            evolvution_fn: Box::new(evolvution_fn),
            neighborhood_fn: Box::new(neighborhood_fn),
        })
    }

    pub fn world(&self) -> Vec<Vec<bool>> {
        self.world.clone()
    }
}

impl<const S: usize, const T: usize> CellularAutomaton for CellularAutomaton2d<S, T> {
    fn step(&mut self) -> usize {
        let prev_world = &self.world.clone();

        let world_size = self.size();

        for i in 0..world_size[0] {
            for j in 0..world_size[1] {
                self.world[i][j] = match (self.neighborhood_fn)(prev_world, i, j) {
                    Neighbors2d::Neighborhood(neighbors) => (self.evolvution_fn)(neighbors),
                    Neighbors2d::Edge => prev_world[i][j],
                };
            }
        }
        self.generation += 1;
        self.generation
    }

    fn age(&self) -> usize {
        self.generation
    }
    fn size(&self) -> Vec<usize> {
        vec![self.world.len(), self.world[0].len()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca2d_flipper() {
        let bv = vec![vec![true, false, false], vec![false, true, true]];
        let mut ca = CellularAutomaton2d::<1, 1>::new(
            bv,
            |x| !x[0][0],
            |world, i, j| Neighbors2d::Neighborhood([[world[i][j]]]),
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![vec![false, true, true], vec![true, false, false]]
        );
    }

    #[test]
    fn test_ca2d_hshifter_loop() {
        let bv = vec![vec![true, false, false], vec![false, true, true]];
        let mut ca = CellularAutomaton2d::<1, 1>::new(
            bv,
            |x| x[0][0],
            |world, i, j| {
                if j == 0 {
                    Neighbors2d::Neighborhood([[world[i][world[0].len() - 1]]])
                } else {
                    Neighbors2d::Neighborhood([[world[i][j - 1]]])
                }
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![vec![false, true, false], vec![true, false, true]]
        );
    }

    #[test]
    fn test_ca2d_vshifter_loop() {
        let bv = vec![vec![true, false, false], vec![false, true, true]];
        let mut ca = CellularAutomaton2d::<1, 1>::new(
            bv,
            |x| x[0][0],
            |world, i, j| {
                if i == 0 {
                    Neighbors2d::Neighborhood([[world[world.len() - 1][j]]])
                } else {
                    Neighbors2d::Neighborhood([[world[i - 1][j]]])
                }
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![vec![false, true, true], vec![true, false, false]]
        );
    }

    #[test]
    fn test_ca2d_xor() {
        let bv = vec![
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ];
        let mut ca = CellularAutomaton2d::<2, 2>::new(
            bv,
            |x| x[0][0] ^ x[0][1] ^ x[1][0] ^ x[1][1],
            |world, i, j| {
                let mut out = [[world[i][j], false], [false, false]];
                let width = world[0].len();
                let height = world.len();
                let in_width = j < (width - 1);
                let in_height = i < (height - 1);

                if in_width {
                    out[0][1] = world[i][j + 1];
                }
                if in_height {
                    out[1][0] = world[i + 1][j];
                }
                if in_width && in_height {
                    out[1][1] = world[i + 1][j + 1];
                }

                Neighbors2d::Neighborhood(out)
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![
                vec![false, false, true],
                vec![false, false, false],
                vec![true, false, true],
            ]
        );
    }
}
