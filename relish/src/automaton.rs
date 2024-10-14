pub trait CellularAutomaton {
    type WorldType;

    fn step(&mut self) -> usize;
    fn size(&self) -> Vec<usize>;
    fn age(&self) -> usize;
    fn world(&self) -> Self::WorldType;
}

#[derive(Debug, Clone)]
pub struct CellularAutomatonWorldSizeError;
