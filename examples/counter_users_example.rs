use bevy::prelude::*;
use iteration_limitation::resources::CounterManager;

pub fn should_print_limited(counter_manager: Res<CounterManager>) -> bool {
    counter_manager.can_increment("prints")
}

pub fn should_spawn_limited(counter_manager: Res<CounterManager>) -> bool {
    counter_manager.can_increment("spawns")
}

fn main() {
    println!("This is an example of counter users logic.");
    println!(
        "Use should_print_limited and should_spawn_limited for conditional printing and spawning."
    );
}
