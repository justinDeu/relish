use core::panic;

pub trait CellularAutomaton {
    fn step(&mut self) -> usize;
}

#[derive(Debug, Clone)]
pub struct CellularAutomaton1dError;

pub struct CellularAutomaton1d<const S: usize> {
    world: Vec<bool>,
    generation: usize,
    evolvution_fn: Box<dyn Fn([bool; S]) -> bool>,
    neighborhood_fn: Box<dyn Fn(&[bool], usize) -> [bool; S]>,
}

impl<const S: usize> CellularAutomaton1d<S> {
    pub fn new(
        world: Vec<bool>,
        evolvution_fn: impl Fn([bool; S]) -> bool + 'static,
        neighborhood_fn: impl Fn(&[bool], usize) -> [bool; S] + 'static,
    ) -> Result<Self, CellularAutomaton1dError> {
        if world.len() < S {
            return Err(CellularAutomaton1dError);
        }

        Ok(Self {
            world,
            generation: 0,
            evolvution_fn: Box::new(evolvution_fn),
            neighborhood_fn: Box::new(neighborhood_fn),
        })
    }

    pub fn width(&self) -> usize {
        self.world.len()
    }

    pub fn age(&self) -> usize {
        self.generation
    }

    pub fn world(&self) -> Vec<bool> {
        self.world.clone()
    }
}

impl<const S: usize> CellularAutomaton for CellularAutomaton1d<S> {
    fn step(&mut self) -> usize {
        let prev_world = &self.world.clone()[..];
        for i in 1..self.width() - 1 {
            // TODO: fix this and rewrite in a way that is clearer and doesn't
            // have an unhandled panic
            let neighbors = (self.neighborhood_fn)(prev_world, i);
            self.world[i] = (self.evolvution_fn)(neighbors);
        }
        self.generation += 1;
        self.generation
    }
}

fn elementary_evolve_builder(pattern: u8) -> impl Fn([bool; 3]) -> bool {
    move |values: [bool; 3]| {
        let mut val = (values[0] as u8) << 2;
        val |= (values[1] as u8) << 1;
        val |= values[2] as u8;

        let (v, _) = pattern.overflowing_shr((val).into());
        (v & 1) != 0
    }
}

fn elementary_neighbor_fn(world: &[bool], i: usize) -> [bool; 3] {
    if i < 1 || i > (world.len() - 1) {
        panic!("elementary_neighbor_fn out of bounds");
    }

    [world[i - 1], world[i], world[i + 1]]
}

#[allow(non_snake_case)]
pub fn ElementaryCellularAutomaton(
    world: Vec<bool>,
    pattern: u8,
) -> Result<CellularAutomaton1d<3>, CellularAutomaton1dError> {
    CellularAutomaton1d::<3>::new(
        world,
        elementary_evolve_builder(pattern),
        elementary_neighbor_fn,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bv = vec![false; 10];
        let result = ElementaryCellularAutomaton(bv, 12);
        assert!(result.is_ok());

        let elem_ca = result.unwrap();
        assert_eq!(elem_ca.width(), 10);
    }

    #[test]
    fn test_step() {
        let mut bv = vec![false; 13];
        bv[6] = true;

        let mut expected = vec![false; 13];
        expected[6] = true;

        let result = ElementaryCellularAutomaton(bv, 30);
        assert!(result.is_ok());

        let mut elem_ca = result.unwrap();
        assert_eq!(elem_ca.world(), expected);

        elem_ca.step();

        expected[5] = true;
        expected[7] = true;

        assert_eq!(elem_ca.world(), expected);

        elem_ca.step();

        expected[4] = true;
        expected[6] = false;
        expected[7] = false;
        expected[8] = true;
        assert_eq!(elem_ca.world(), expected);
    }

    #[test]
    fn test_invalid_world_size() {
        let bv = vec![true, false];
        let result = ElementaryCellularAutomaton(bv, 1);

        assert!(result.is_err());
    }
}
