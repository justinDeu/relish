pub trait CellularAutomaton {
    fn step(&mut self) -> usize;
    fn size(&self) -> Vec<usize>;
    fn age(&self) -> usize;
}

#[derive(Debug, Clone)]
pub struct CellularAutomatonWorldSizeError;
