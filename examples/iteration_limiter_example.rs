use bevy::prelude::*;
use iteration_limitation::resources::{Counter, MaxIterations};

pub fn should_continue_iterations(
    counter: Res<Counter>,
    max_iterations: Res<MaxIterations>,
) -> bool {
    counter.0 < max_iterations.0
}

fn main() {
    println!("This is an example of iteration limiting logic.");
    println!("Use should_continue_iterations to limit how many times a system runs.");
}
