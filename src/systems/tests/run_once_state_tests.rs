use crate::resources::RunOnceState;
use bevy::prelude::*;

#[cfg(test)]
mod run_once_state_tests {
    use super::*;

    #[test]
    fn test_run_once_state_initial_state() {
        let state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };
        assert!(!state.run_once_state_a);
        assert!(!state.run_once_state_b);
    }

    #[test]
    fn test_run_once_state_mark_a_as_run() {
        let mut state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };

        // Initially should run
        assert!(!state.run_once_state_a);

        // Mark as run
        state.run_once_state_a = true;

        // Now should not run again
        assert!(state.run_once_state_a);

        // B should still be unaffected
        assert!(!state.run_once_state_b);
    }

    #[test]
    fn test_run_once_state_mark_b_as_run() {
        let mut state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };

        // Initially should run
        assert!(!state.run_once_state_b);

        // Mark as run
        state.run_once_state_b = true;

        // Now should not run again
        assert!(state.run_once_state_b);

        // A should still be unaffected
        assert!(!state.run_once_state_a);
    }

    #[test]
    fn test_run_once_state_reset_a() {
        let mut state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };

        // Mark A as run
        state.run_once_state_a = true;
        assert!(state.run_once_state_a);

        // Reset A
        state.run_once_state_a = false;

        // A should be back to initial state
        assert!(!state.run_once_state_a);

        // B should still be unaffected
        assert!(!state.run_once_state_b);
    }

    #[test]
    fn test_run_once_state_reset_b() {
        let mut state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };

        // Mark B as run
        state.run_once_state_b = true;
        assert!(state.run_once_state_b);

        // Reset B
        state.run_once_state_b = false;

        // B should be back to initial state
        assert!(!state.run_once_state_b);

        // A should still be unaffected
        assert!(!state.run_once_state_a);
    }

    #[test]
    fn test_run_once_state_independent_operations() {
        let mut state = RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        };

        // Test various combinations of operations
        assert!(!state.run_once_state_a);
        assert!(!state.run_once_state_b);

        // Mark A as run
        state.run_once_state_a = true;
        assert!(state.run_once_state_a);
        assert!(!state.run_once_state_b);

        // Mark B as run
        state.run_once_state_b = true;
        assert!(state.run_once_state_a);
        assert!(state.run_once_state_b);

        // Reset A only
        state.run_once_state_a = false;
        assert!(!state.run_once_state_a);
        assert!(state.run_once_state_b);

        // Reset B only
        state.run_once_state_b = false;
        assert!(!state.run_once_state_a);
        assert!(!state.run_once_state_b);
    }

    // Integration test with Bevy systems
    #[test]
    fn test_run_once_state_in_bevy_app() {
        let mut app = App::new();

        // Insert the resource
        app.insert_resource(RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        });

        // Add test systems
        app.add_systems(Update, test_action_a_system);
        app.add_systems(Update, test_action_b_system);

        // Run once - both should execute
        app.update();
        let state = app.world().resource::<RunOnceState>();
        assert!(state.run_once_state_a);
        assert!(state.run_once_state_b);

        // Run again - neither should execute again
        app.update();
        let state = app.world().resource::<RunOnceState>();
        assert!(state.run_once_state_a);
        assert!(state.run_once_state_b);

        // Reset A and run again - only A should execute
        app.world_mut().resource_mut::<RunOnceState>().run_once_state_a = false;
        app.update();
        let state = app.world().resource::<RunOnceState>();
        assert!(state.run_once_state_a); // A ran again
        assert!(state.run_once_state_b); // B still marked as run
    }

    // Helper systems for integration test
    fn test_action_a_system(mut state: ResMut<RunOnceState>) {
        if !state.run_once_state_a {
            state.run_once_state_a = true;
        }
    }

    fn test_action_b_system(mut state: ResMut<RunOnceState>) {
        if !state.run_once_state_b {
            state.run_once_state_b = true;
        }
    }
}