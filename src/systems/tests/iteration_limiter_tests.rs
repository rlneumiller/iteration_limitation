
use bevy::prelude::*;
use crate::resources::{Counter, MaxIterations};
use crate::systems::iteration_limiter::iteration_limiter_system;

#[test]
fn test_iteration_limiter_system() {
    let mut app = App::new();
    app.insert_resource(Counter(0));
    app.insert_resource(MaxIterations(3));
    app.add_systems(Update, iteration_limiter_system);

    // Run 1: Counter should be 1
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 1);

    // Run 2: Counter should be 2
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 2);

    // Run 3: Counter should be 3 (max iterations)
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 3);

    // Run 4: Counter should still be 3 (limit reached)
    app.update();
    assert_eq!(app.world().resource::<Counter>().0, 3);
}
