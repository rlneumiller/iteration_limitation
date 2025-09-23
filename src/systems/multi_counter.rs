use crate::resources::*;
use bevy::prelude::*;

pub fn multi_counter_system(mut counter_manager: ResMut<CounterManager>) {
    // Initialize counters if not already done
    if counter_manager.counters.is_empty() {
        counter_manager.add_counter("prints", 3);
        counter_manager.add_counter("spawns", 5);
        counter_manager.add_counter("effects", 2);
    }

    // Example: Increment different counters
    if counter_manager.increment("prints") {
        println!(
            "Print counter: {}/{}",
            counter_manager.get_count("prints"),
            counter_manager.get_limit("prints")
        );
    }

    if counter_manager.increment("spawns") {
        println!(
            "Spawn counter: {}/{}",
            counter_manager.get_count("spawns"),
            counter_manager.get_limit("spawns")
        );
    }

    if counter_manager.increment("effects") {
        println!(
            "Effect counter: {}/{}",
            counter_manager.get_count("effects"),
            counter_manager.get_limit("effects")
        );
    }
}
