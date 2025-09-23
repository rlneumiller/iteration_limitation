//! 💬 For questions or feedback, visit: https://github.com/rlneumiller/iteration_limitation/discussions

use bevy::prelude::*;
use iteration_limitation::IterationControlPlugin;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_plugins(IterationControlPlugin);

    app.run();
}
