use crate::automaton::{CellularAutomaton, CellularAutomatonWorldSizeError};

pub enum Neighbors1d<const S: usize> {
    Neighborhood([bool; S]),
    Edge,
}

pub struct CellularAutomaton1d<const S: usize> {
    world: Vec<bool>,
    generation: usize,
    evolvution_fn: Box<dyn Fn([bool; S]) -> bool>,
    neighborhood_fn: Box<dyn Fn(&[bool], usize) -> Neighbors1d<S>>,
}

impl<const S: usize> CellularAutomaton1d<S> {
    pub fn new(
        world: Vec<bool>,
        evolvution_fn: impl Fn([bool; S]) -> bool + 'static,
        neighborhood_fn: impl Fn(&[bool], usize) -> Neighbors1d<S> + 'static,
    ) -> Result<Self, CellularAutomatonWorldSizeError> {
        if world.len() < S {
            return Err(CellularAutomatonWorldSizeError);
        }

        Ok(Self {
            world,
            generation: 0,
            evolvution_fn: Box::new(evolvution_fn),
            neighborhood_fn: Box::new(neighborhood_fn),
        })
    }

    pub fn world(&self) -> Vec<bool> {
        self.world.clone()
    }
}

impl<const S: usize> CellularAutomaton for CellularAutomaton1d<S> {
    fn step(&mut self) -> usize {
        let prev_world = &self.world.clone()[..];

        self.world = (0..self.size()[0])
            .map(|i| match (self.neighborhood_fn)(prev_world, i) {
                Neighbors1d::Neighborhood(neighbors) => (self.evolvution_fn)(neighbors),
                Neighbors1d::Edge => prev_world[i],
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
}

/*
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ca1d_flipper() {
        let bv = vec![false; 10];
        let mut ca = CellularAutomaton1d::<1>::new(
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
        let mut ca = CellularAutomaton1d::<1>::new(
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
        let mut ca = CellularAutomaton1d::<2>::new(
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
