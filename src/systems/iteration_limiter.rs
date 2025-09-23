use crate::resources::*;
use bevy::prelude::*;

pub fn iteration_limiter_system(mut counter: ResMut<Counter>, max_iterations: Res<MaxIterations>) {
    if counter.0 < max_iterations.0 {
        counter.0 += 1;
    }
    // Note: No printing here - this system just manages the counter
    // Other systems can check the counter to decide whether to print
}
