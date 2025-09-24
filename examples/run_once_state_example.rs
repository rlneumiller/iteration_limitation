use bevy::prelude::*;
use iteration_limitation::resources::RunOnceState;

/// System that performs action A only once
fn action_a_system(mut run_once_state: ResMut<RunOnceState>) {
    if !run_once_state.run_once_state_a {
        println!("🚀 Action A executed! This will only run once.");
        run_once_state.run_once_state_a = true;
    } else {
        println!("⏭️  Action A system called but already ran.");
    }
}

/// System that performs action B only once
fn action_b_system(mut run_once_state: ResMut<RunOnceState>) {
    if !run_once_state.run_once_state_b {
        println!("🎯 Action B executed! This will only run once.");
        run_once_state.run_once_state_b = true;
    } else {
        println!("⏭️  Action B system called but already ran.");
    }
}

/// System that demonstrates both actions can be reset independently
fn reset_system(mut run_once_state: ResMut<RunOnceState>, time: Res<Time>) {
    // Reset action A every 5 seconds
    if time.elapsed_secs() > 5.0 && time.elapsed_secs() < 5.1 {
        println!("🔄 Resetting Action A...");
        run_once_state.run_once_state_a = false;
    }

    // Reset action B every 10 seconds
    if time.elapsed_secs() > 10.0 && time.elapsed_secs() < 10.1 {
        println!("🔄 Resetting Action B...");
        run_once_state.run_once_state_b = false;
    }
}

/// System that shows the current state
fn status_system(run_once_state: Res<RunOnceState>, time: Res<Time>) {
    if time.elapsed_secs().fract() < 0.1 { // Print status roughly every second
        println!("📊 Status - A: {}, B: {} (elapsed: {:.1}s)",
                 if run_once_state.run_once_state_a { "✅" } else { "⏳" },
                 if run_once_state.run_once_state_b { "✅" } else { "⏳" },
                 time.elapsed_secs());
    }
}

fn main() {
    println!("🎮 RunOnceState Example");
    println!("This example demonstrates two independent run-once actions.");
    println!("- Action A runs once, then gets reset every 5 seconds");
    println!("- Action B runs once, then gets reset every 10 seconds");
    println!("Watch the status updates to see the independent behavior!\n");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(RunOnceState {
            run_once_state_a: false,
            run_once_state_b: false,
        })
        .add_systems(Update, (
            action_a_system,
            action_b_system,
            reset_system,
            status_system,
        ))
        .run();
}