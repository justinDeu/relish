use crate::automaton::{CellularAutomaton, CellularAutomatonWorldSizeError};

pub enum Neighbors1d<CellType, const WIDTH: usize> {
    Neighborhood([CellType; WIDTH]),
    Edge,
}

pub struct CellularAutomaton1d<CellType: Clone, const WIDTH: usize> {
    world: Vec<CellType>,
    generation: usize,
    evolvution_fn: Box<dyn Fn([CellType; WIDTH]) -> CellType>,
    neighborhood_fn: Box<dyn Fn(&[CellType], usize) -> Neighbors1d<CellType, WIDTH>>,
}

impl<CellType: Clone, const WIDTH: usize> CellularAutomaton1d<CellType, WIDTH> {
    pub fn new(
        world: Vec<CellType>,
        evolvution_fn: impl Fn([CellType; WIDTH]) -> CellType + 'static,
        neighborhood_fn: impl Fn(&[CellType], usize) -> Neighbors1d<CellType, WIDTH> + 'static,
    ) -> Result<Self, CellularAutomatonWorldSizeError> {
        if world.len() < WIDTH {
            return Err(CellularAutomatonWorldSizeError);
        }

        Ok(Self {
            world,
            generation: 0,
            evolvution_fn: Box::new(evolvution_fn),
            neighborhood_fn: Box::new(neighborhood_fn),
        })
    }
}

impl<CellType: Clone, const WIDTH: usize> CellularAutomaton
    for CellularAutomaton1d<CellType, WIDTH>
{
    type WorldType = Vec<CellType>;

    fn step(&mut self) -> usize {
        let prev_world = &self.world.clone()[..];

        self.world = (0..self.size()[0])
            .map(|i| match (self.neighborhood_fn)(prev_world, i) {
                Neighbors1d::Neighborhood(neighbors) => (self.evolvution_fn)(neighbors),
                Neighbors1d::Edge => prev_world[i].clone(),
            })
            .collect();
        self.generation += 1;
        self.generation
    }

    fn size(&self) -> Vec<usize> {
        vec![self.world.len()]
    }

    fn age(&self) -> usize {
        self.generation
    }
    fn world(&self) -> Self::WorldType {
        self.world.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca1d_flipper() {
        let bv = vec![false; 10];
        let mut ca = CellularAutomaton1d::<bool, 1>::new(
            bv,
            |x| !x[0],
            |world, i| Neighbors1d::Neighborhood([world[i]]),
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(ca.world(), vec![true; 10]);
    }

    #[test]
    fn test_ca1d_shifter() {
        let bv = vec![false, true, false, true];
        let mut ca = CellularAutomaton1d::<bool, 1>::new(
            bv,
            |x| x[0],
            |world, i| {
                if i == 0 {
                    Neighbors1d::Edge
                } else {
                    Neighbors1d::Neighborhood([world[i - 1]])
                }
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(ca.world(), vec![false, false, true, false]);
    }

    #[test]
    fn test_ca1d_and_prev() {
        let bv = vec![false, true, true, false];
        let mut ca = CellularAutomaton1d::<bool, 2>::new(
            bv,
            |x| x[0] && x[1],
            |world, i| {
                if i == 0 {
                    Neighbors1d::Edge
                } else {
                    Neighbors1d::Neighborhood([world[i - 1], world[i]])
                }
            },
        )
        .expect("Construction failed");

        ca.step();

        assert_eq!(ca.world(), vec![false, false, true, false]);
    }
}
