use relish::automaton::CellularAutomaton;
use relish::conway::ConwayCellularAutomaton;
use relish::dim2::CellularAutomaton2d;

fn print_conway(conway: &CellularAutomaton2d<bool, 3, 3>) {
    let shape = conway.size();
    let world = conway.world();
    print!("   ");
    for i in 1..shape[1] + 1 {
        print!("{:>}", i);
    }
    println!();
    for (i, row) in world.iter().enumerate() {
        print!("{:>2}|", i + 1);
        row.iter().for_each(|x| match x {
            true => print!("\u{2588}"),
            false => print!(" "),
        });
        println!();
    }
}

fn main() {
    let mut world = vec![vec![false; 9]; 9];

    world[2][3] = true;
    world[2][6] = true;
    world[3][2] = true;
    world[4][2] = true;
    world[4][6] = true;
    world[5][2] = true;
    world[5][3] = true;
    world[5][4] = true;
    world[5][5] = true;

    let mut ca = ConwayCellularAutomaton(world, true).expect("Construction failed");

    for _ in 0..20 {
        print_conway(&ca);
        ca.step();
    }
}
