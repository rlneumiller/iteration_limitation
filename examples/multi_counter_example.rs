use bevy::prelude::*;
use iteration_limitation::resources::CounterManager;

pub fn can_increment_prints(counter_manager: Res<CounterManager>) -> bool {
    counter_manager.can_increment("prints")
}

pub fn can_increment_spawns(counter_manager: Res<CounterManager>) -> bool {
    counter_manager.can_increment("spawns")
}

pub fn can_increment_effects(counter_manager: Res<CounterManager>) -> bool {
    counter_manager.can_increment("effects")
}

fn main() {
    println!("This is an example of multi-counter logic.");
    println!("Use the can_increment_* functions to check if specific counters can be incremented.");
}
