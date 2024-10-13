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
