#[allow(unused_imports)]
use crate::automaton::CellularAutomaton;

use crate::automaton::CellularAutomatonWorldSizeError;

use crate::dim1::{CellularAutomaton1d, Neighbors1d};

fn elementary_evolve_builder(pattern: u8) -> impl Fn([bool; 3]) -> bool {
    move |values: [bool; 3]| {
        let mut val = (values[0] as u8) << 2;
        val |= (values[1] as u8) << 1;
        val |= values[2] as u8;

        let (v, _) = pattern.overflowing_shr((val).into());
        (v & 1) != 0
    }
}

fn elementary_neighbor_fn(world: &[bool], i: usize) -> Neighbors1d<3> {
    if i < 1 || i > (world.len() - 2) {
        return Neighbors1d::Edge;
    }

    Neighbors1d::Neighborhood([world[i - 1], world[i], world[i + 1]])
}

#[allow(non_snake_case)]
pub fn ElementaryCellularAutomaton(
    world: Vec<bool>,
    pattern: u8,
) -> Result<CellularAutomaton1d<3>, CellularAutomatonWorldSizeError> {
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
        assert_eq!(elem_ca.size(), vec![10]);
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
