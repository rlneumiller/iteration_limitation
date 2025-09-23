use bevy::prelude::*;
use iteration_limitation::resources::{Counter, MaxIterations};

pub fn can_print(counter: Res<Counter>, max_iterations: Res<MaxIterations>) -> bool {
    counter.0 < max_iterations.0
}

fn main() {
    println!("This is an example of limited printing logic.");
    println!("Use can_print to conditionally print messages based on iteration count.");
}
