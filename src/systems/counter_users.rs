use crate::resources::*;
use bevy::prelude::*;

pub fn print_limiter_system(counter_manager: Res<CounterManager>) {
    if counter_manager.can_increment("prints") {
        println!("Limited print from dedicated system!");
    } else {
        println!("Print limit reached in dedicated system.");
    }
}

pub fn spawn_limiter_system(counter_manager: Res<CounterManager>) {
    if counter_manager.can_increment("spawns") {
        println!(
            "Entity spawned! ({}/{})",
            counter_manager.get_count("spawns") + 1,
            counter_manager.get_limit("spawns")
        );
    } else {
        println!("Spawn limit reached.");
    }
}
