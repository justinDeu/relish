use relish::{ElementaryCA, CA};

const NUM_STEPS: usize = 40;
const WORLD_SIZE: usize = 50;
const PATTERN: u8 = 30;

fn print_world(world: &[bool]) {
    world.iter().for_each(|x| match x {
        true => print!("\u{2588}"),
        false => print!(" "),
    });
}

fn main() {
    let mut bv = vec![false; WORLD_SIZE];
    bv[WORLD_SIZE.div_ceil(2) + 1] = true;

    let mut ca = ElementaryCA::new(bv, PATTERN);
    print!("{:>2}\u{2595}", ca.age());
    print_world(&ca.world()[..]);
    println!();

    for _ in 0..NUM_STEPS {
        ca.step();
        print!("{:>2}\u{2595}", ca.age());
        print_world(&ca.world()[..]);
        println!();
    }
}
