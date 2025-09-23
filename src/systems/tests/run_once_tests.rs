
use bevy::prelude::*;
use crate::resources::HasRun;
use crate::systems::run_once::run_once_system;

#[test]
fn test_run_once_system() {
    let mut app = App::new();
    app.insert_resource(HasRun(false));
    app.add_systems(Update, run_once_system);

    // First run: system should execute and set HasRun to true
    app.update();
    assert_eq!(app.world().resource::<HasRun>().0, true);

    // Second run: system should not execute, HasRun should remain true
    app.update();
    assert_eq!(app.world().resource::<HasRun>().0, true);
}
