use bevy::prelude::*;
use crate::systems::once_static::once_static_system;

#[test]
fn test_once_static_system() {
    let mut app = App::new();
    app.add_systems(Update, once_static_system);

    // Run the app multiple times - the Once should ensure the closure runs only once
    // Since it's static, we can't easily assert the internal state, but we can ensure no panics
    app.update();
    app.update();
    app.update();

    // The system should not panic and the app should continue running
    assert!(true); // Basic assertion that we reached this point
}

#[test]
fn test_once_static_system_in_isolation() {
    // Test that the system can be added to an app without issues
    let mut app = App::new();
    app.add_systems(Update, once_static_system);

    // Just ensure the app builds and updates without error
    app.update();
}