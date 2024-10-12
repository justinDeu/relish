# Relish

Learning about Rust libraries and writing some Cellular Automata

## Things to Look Into/Do

[ ] Generalize 1d away from only Elementary and allow arbitrary function to be provided
[ ] Add basic drawing libraries
  - [ ] simple print drawer
  - [ ] Ratatiu drawer
[ ] Add `world` abstraction that allows setting time interval
[ ] Add ability to save off CA to file and load
[ ] Add dimensions of CA's world into type system (ie: Elementary CA is 1, Darwin and WireWorld are 2)
  - [ ] Generalize Drawers to behave differently based on CA type
  - [ ] Ratatui always does full replacement for 2d world
  - [ ] Option to replace or waterfall in 1d
