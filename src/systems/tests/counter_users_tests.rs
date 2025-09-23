
use bevy::prelude::*;
use crate::resources::CounterManager;
use crate::systems::counter_users::{print_limiter_system, spawn_limiter_system};

#[test]
fn test_print_limiter_system_conditions() {
    // Test when can_increment is true
    let mut app = App::new();
    app.insert_resource(CounterManager::new());
    app.world_mut().resource_mut::<CounterManager>().add_counter("prints", 1);
    app.add_systems(Update, print_limiter_system);
    app.update();
    assert_eq!(app.world().resource::<CounterManager>().get_count("prints"), 0); // Should not increment

    // Test when can_increment is false (limit reached)
    let mut app = App::new();
    app.insert_resource(CounterManager::new());
    app.world_mut().resource_mut::<CounterManager>().add_counter("prints", 1);
    app.world_mut().resource_mut::<CounterManager>().increment("prints"); // Manually increment to reach limit
    app.add_systems(Update, print_limiter_system);
    app.update();
    assert_eq!(app.world().resource::<CounterManager>().get_count("prints"), 1); // Should still be at limit
}

#[test]
fn test_spawn_limiter_system_conditions() {
    // Test when can_increment is true
    let mut app = App::new();
    app.insert_resource(CounterManager::new());
    app.world_mut().resource_mut::<CounterManager>().add_counter("spawns", 1);
    app.add_systems(Update, spawn_limiter_system);
    app.update();
    assert_eq!(app.world().resource::<CounterManager>().get_count("spawns"), 0); // Should not increment

    // Test when can_increment is false (limit reached)
    let mut app = App::new();
    app.insert_resource(CounterManager::new());
    app.world_mut().resource_mut::<CounterManager>().add_counter("spawns", 1);
    app.world_mut().resource_mut::<CounterManager>().increment("spawns"); // Manually increment to reach limit
    app.add_systems(Update, spawn_limiter_system);
    app.update();
    assert_eq!(app.world().resource::<CounterManager>().get_count("spawns"), 1); // Should still be at limit
}
