//! 💬 For questions or feedback, visit: <https://github.com/rlneumiller/iteration_limitation/discussions>

use bevy::prelude::*;
use crate::resources::*;

pub mod resources;
pub mod systems;

pub mod prelude {
    pub use crate::resources::*;
    pub use crate::systems::*;
}

/// Plugin that provides iteration control systems for Bevy applications.
///
/// This plugin includes systems for:
/// - Run-once execution
/// - Global once-time initialization using std::sync::Once
/// - Iteration limiting
/// - Multi-counter management
/// - Conditional execution based on counters
///
/// # Example
/// ```rust,no_run
/// use bevy::prelude::*;
/// use iteration_limitation::IterationControlPlugin;
///
/// fn main() {
///     App::new()
///         .add_plugins(DefaultPlugins)
///         .add_plugins(IterationControlPlugin)
///         .run();
/// }
/// ```
pub struct IterationControlPlugin;

impl Plugin for IterationControlPlugin {
    fn build(&self, app: &mut App) {
        // Insert default resources
        app.insert_resource(HasRun(false));
        app.insert_resource(MaxIterations(5));
        app.insert_resource(Counter(0));
        app.insert_resource(CounterManager::new());

        // Add systems
        app.add_systems(Update, systems::run_once::run_once_system);
        app.add_systems(Update, systems::iteration_limiter::iteration_limiter_system);
        app.add_systems(Update, systems::limited_print::limited_print_system);
        app.add_systems(Update, systems::multi_counter::multi_counter_system);
        app.add_systems(Update, systems::counter_users::print_limiter_system);
        app.add_systems(Update, systems::counter_users::spawn_limiter_system);
        app.add_systems(Update, systems::counter_reset::counter_reset_demo);
        app.add_systems(Update, systems::once_static::once_static_system);
    }
}
