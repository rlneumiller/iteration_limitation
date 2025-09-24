use bevy::prelude::*;
use iteration_limitation::resources::HasRun;

pub fn should_run(has_run: Res<HasRun>) -> bool {
    !has_run.0
}

fn main() {
    println!("This is an example demonstrating the use of single global run-once logic.");
    println!("Use should_run as a run condition to execute a system only once.");
}
