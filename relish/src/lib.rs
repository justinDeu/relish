pub trait CA {
    fn step(&mut self) -> usize;
}

pub struct ElementaryCA {
    world: Vec<bool>,
    prev_world: Vec<bool>,
    generation: usize,
    pattern: u8,
}

#[derive(Debug, Clone)]
pub struct ElementaryCAError;

impl ElementaryCA {
    pub fn new(world: Vec<bool>, pattern: u8) -> Result<Self, ElementaryCAError> {
        if (world.len()) < 3 {
            return Err(ElementaryCAError);
        }

        let prev_world = world.clone();
        Ok(Self {
            world,
            prev_world,
            generation: 0,
            pattern,
        })
    }

    fn evolve(&self, values: [bool; 3]) -> bool {
        let mut val = (values[0] as u8) << 2;
        val |= (values[1] as u8) << 1;
        val |= values[2] as u8;

        let (v, _) = self.pattern.overflowing_shr((val).into());
        (v & 1) != 0
    }

    pub fn width(&self) -> usize {
        self.world.len()
    }

    pub fn age(&self) -> usize {
        self.generation
    }

    pub fn pattern(&self) -> u8 {
        self.pattern
    }

    pub fn world(&self) -> Vec<bool> {
        self.world.clone()
    }
}

impl CA for ElementaryCA {
    fn step(&mut self) -> usize {
        self.prev_world = self.world.clone();
        for i in 1..self.width() - 1 {
            // TODO: fix this and rewrite in a way that is clearer and doesn't
            // have an unhandled panic
            self.world[i] = self.evolve(*self.prev_world[i - 1..].first_chunk::<3>().unwrap());
        }
        self.generation += 1;
        self.generation
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bv = vec![false; 10];
        let result = ElementaryCA::new(bv, 12);
        assert!(result.is_ok());

        let elem_ca = result.unwrap();
        assert_eq!(elem_ca.width(), 10);
        assert_eq!(elem_ca.pattern(), 12);
    }

    #[test]
    fn test_step() {
        let mut bv = vec![false; 13];
        bv[6] = true;

        let mut expected = vec![false; 13];
        expected[6] = true;

        let result = ElementaryCA::new(bv, 30);
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
        let result = ElementaryCA::new(bv, 1);

        assert!(result.is_err());
    }
}
