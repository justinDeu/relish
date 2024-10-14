use crate::automaton::{CellularAutomaton, CellularAutomatonWorldSizeError};

pub enum Neighbors2d<CellType, const HEIGHT: usize, const WIDTH: usize> {
    Neighborhood([[CellType; WIDTH]; HEIGHT]),
    Edge,
}

pub struct CellularAutomaton2d<CellType: Clone, const HEIGHT: usize, const WIDTH: usize> {
    world: Vec<Vec<CellType>>,
    generation: usize,
    evolvution_fn: Box<dyn Fn([[CellType; WIDTH]; HEIGHT]) -> CellType>,
    neighborhood_fn:
        Box<dyn Fn(&Vec<Vec<CellType>>, usize, usize) -> Neighbors2d<CellType, HEIGHT, WIDTH>>,
}

impl<CellType: Clone, const HEIGHT: usize, const WIDTH: usize>
    CellularAutomaton2d<CellType, HEIGHT, WIDTH>
{
    pub fn new(
        world: Vec<Vec<CellType>>,
        evolvution_fn: impl Fn([[CellType; WIDTH]; HEIGHT]) -> CellType + 'static,
        neighborhood_fn: impl Fn(&Vec<Vec<CellType>>, usize, usize) -> Neighbors2d<CellType, HEIGHT, WIDTH>
            + 'static,
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
}

impl<CellType: Clone, const S: usize, const T: usize> CellularAutomaton
    for CellularAutomaton2d<CellType, S, T>
{
    type WorldType = Vec<Vec<CellType>>;
    fn step(&mut self) -> usize {
        let prev_world = &self.world.clone();

        let world_size = self.size();

        for i in 0..world_size[0] {
            for j in 0..world_size[1] {
                self.world[i][j] = match (self.neighborhood_fn)(prev_world, i, j) {
                    Neighbors2d::Neighborhood(neighbors) => (self.evolvution_fn)(neighbors),
                    Neighbors2d::Edge => prev_world[i][j].clone(),
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

    fn world(&self) -> Self::WorldType {
        self.world.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca2d_flipper() {
        let bv = vec![vec![true, false, false], vec![false, true, true]];
        let mut ca = CellularAutomaton2d::<bool, 1, 1>::new(
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
        let mut ca = CellularAutomaton2d::<bool, 1, 1>::new(
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
        let mut ca = CellularAutomaton2d::<bool, 1, 1>::new(
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
        let mut ca = CellularAutomaton2d::<bool, 2, 2>::new(
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

    #[test]
    fn test_ca2d_offset() {
        let bv = vec![
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ];
        let mut ca = CellularAutomaton2d::<bool, 1, 2>::new(
            bv,
            |x| x[0][0] ^ x[0][1],
            |world, i, j| {
                let mut out = [[world[i][j], false]];
                let width = world[0].len();

                if j < (width - 1) {
                    out[0][1] = world[i][j + 1];
                }

                Neighbors2d::Neighborhood(out)
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![
                vec![true, false, false],
                vec![true, false, true],
                vec![true, false, true],
            ]
        );
    }

    #[test]
    fn test_ca2d_offset2() {
        let bv = vec![
            vec![true, false, false],
            vec![false, true, true],
            vec![false, true, true],
        ];
        let mut ca = CellularAutomaton2d::<bool, 2, 1>::new(
            bv,
            |x| x[0][0] ^ x[1][0],
            |world, i, j| {
                let mut out = [[world[i][j]], [false]];
                let height = world.len();

                if i < (height - 1) {
                    out[1][0] = world[i + 1][j];
                }

                Neighbors2d::Neighborhood(out)
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(
            ca.world(),
            vec![
                vec![true, true, true],
                vec![false, false, false],
                vec![false, true, true],
            ]
        );
    }
}
