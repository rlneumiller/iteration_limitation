use crate::resources::*;
use bevy::prelude::*;

pub fn limited_print_system(counter: Res<Counter>, max_iterations: Res<MaxIterations>) {
    // Only print if we haven't reached the maximum iterations
    if counter.0 < max_iterations.0 {
        println!("Limited print statement #{}", counter.0 + 1);
    } else {
        // Optional: print once when limit is reached
        if counter.0 == max_iterations.0 {
            println!("Print limit reached! No more prints allowed.");
        }
    }
}
