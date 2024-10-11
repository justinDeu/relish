pub trait CA {
    fn step(&mut self) -> usize;
}

pub struct ElementaryCA {
    world: Vec<bool>,
    prev_world: Vec<bool>,
    age: usize,
    pattern: u8,
}

impl ElementaryCA {
    pub fn new(world: Vec<bool>, pattern: u8) -> Self {
        let prev_world = world.clone();
        Self {
            world,
            prev_world,
            age: 0,
            pattern,
        }
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
        self.age
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
        self.age += 1;
        self.age
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let bv = vec![false; 10];
        let elem_ca = ElementaryCA::new(bv, 12);
        assert_eq!(elem_ca.width(), 10);
        assert_eq!(elem_ca.pattern(), 12);
    }

    #[test]
    fn test_step() {
        let mut bv = vec![false; 13];
        bv[6] = true;

        let mut expected = vec![false; 13];
        expected[6] = true;

        let mut elem_ca = ElementaryCA::new(bv, 30);

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
}
