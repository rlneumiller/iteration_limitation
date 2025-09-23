# iteration_limitation

A reusable Bevy library providing various iteration control systems including run-once execution, iteration limiting, and multi-counter management.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
iteration_limitation = "0.1.0"
```

## Quick Start

The easiest way to get started is to use the `IterationControlPlugin`:

```rust
use bevy::prelude::*;
use iteration_limitation::IterationControlPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(IterationControlPlugin)
        .run();
}
```

This adds all systems and default resources to your Bevy app.

## Manual Usage

If you prefer more control, you can add individual systems and resources:

```rust
use bevy::prelude::*;
use iteration_limitation::prelude::*;

fn main() {
    let mut app = App::new()
        .add_plugins(DefaultPlugins);

    // Add resources
    app.insert_resource(HasRun(false));
    app.insert_resource(MaxIterations(10));
    app.insert_resource(Counter(0));
    app.insert_resource(CounterManager::new());

    // Add systems
    app.add_systems(Update, run_once::run_once_system);
    app.add_systems(Update, iteration_limiter::iteration_limiter_system);
    // ... add other systems as needed

    app.run();
}
```

## Systems Overview

This project includes several systems for controlling execution frequency and iteration limits:

- **Run Once System**: Executes a system only once during the application lifetime
- **Iteration Limiter**: Limits the number of times a behavior can occur
- **Limited Print**: Demonstrates conditional printing based on iteration count
- **Multi Counter**: Manages multiple independent counters with different limits
- **Counter Users**: Shows how different systems can use counter limits
- **Counter Reset**: Demonstrates resetting counters based on conditions

## Run Once System

The simplest form of iteration control - execute something exactly once.

### How the Run Once System Works

Uses a `HasRun` resource to track execution state:

```rust
#[derive(Resource)]
pub struct HasRun(pub bool);
```

### Usage

```rust
pub fn run_once_system(mut has_run: ResMut<HasRun>) {
    if !has_run.0 {
        println!("Running once during gameplay!");
        has_run.0 = true;
    }
}
```

### Conditional Execution

Use condition functions with `.run_if()`:

```rust
app.add_systems(Update, my_system.run_if(should_run));
```

Where `should_run` checks the `HasRun` state.

## Iteration Limiting System

This project includes a flexible system for limiting the number of iterations of repetitive behaviors, such as print statements, based on a configurable resource value.

### How It Works

The system uses two resources:

- `MaxIterations`: Defines the maximum number of iterations allowed
- `Counter`: Tracks the current iteration count

### Usage Pattern

1. **Counter Management**: The `iteration_limiter_system` increments a counter each frame up to the maximum value, acting as a global limiter.

2. **Limited Behavior**: Other systems can check the counter value to conditionally execute limited behaviors:

```rust
pub fn my_limited_system(counter: Res<Counter>, max_iterations: Res<MaxIterations>) {
    if counter.0 < max_iterations.0 {
        // Your limited behavior here (prints, actions, etc.)
        println!("This will only execute {} times", max_iterations.0);
    }
}
```

### Example: Limiting Print Statements

The `limited_print_system` demonstrates this pattern by printing numbered statements only while the counter is below the limit:

```rust
pub fn limited_print_system(counter: Res<Counter>, max_iterations: Res<MaxIterations>) {
    if counter.0 < max_iterations.0 {
        println!("Limited print statement #{}", counter.0 + 1);
    } else {
        // Optional: notify when limit is reached
        if counter.0 == max_iterations.0 {
            println!("Print limit reached! No more prints allowed.");
        }
    }
}
```

### Configuration

Adjust the maximum iterations by changing the `MaxIterations` resource:

```rust
app.insert_resource(MaxIterations(10)); // Allow 10 iterations
```

### Benefits

- **Decoupled**: The limiter system manages counting independently of consuming systems
- **Reusable**: Multiple systems can reference the same counter for different purposes
- **Configurable**: Easy to adjust limits without changing system logic
- **Thread-safe**: Uses Bevy's resource system for safe concurrent access

### Alternative Approaches

1. **Conditional System Execution**: Use `.run_if()` with condition functions
2. **Multiple Counters**: Create separate counter resources for different types of limits
3. **Reset Logic**: Add conditions to reset counters (e.g., based on time, events, or game state)

## Multiple Counters

For more complex scenarios requiring different limits for different behaviors, you can use the `CounterManager` resource to manage multiple named counters.

### Setup

Initialize the `CounterManager` and add named counters with their limits:

```rust
app.insert_resource(CounterManager::new());
```

### Adding Counters

In a startup system or during initialization:

```rust
pub fn setup_counters(mut counter_manager: ResMut<CounterManager>) {
    counter_manager.add_counter("prints", 3);      // Limit prints to 3
    counter_manager.add_counter("spawns", 5);      // Limit spawns to 5
    counter_manager.add_counter("effects", 2);     // Limit effects to 2
}
```

### Using Multiple Counters

Different systems can use different counters:

```rust
pub fn print_system(counter_manager: Res<CounterManager>) {
    if counter_manager.can_increment("prints") {
        println!("This print is limited to {} times", counter_manager.get_limit("prints"));
    }
}

pub fn spawn_system(mut counter_manager: ResMut<CounterManager>) {
    if counter_manager.increment("spawns") {
        // Spawn entity logic here
        println!("Spawned entity {}/{}", 
                 counter_manager.get_count("spawns"), 
                 counter_manager.get_limit("spawns"));
    }
}
```

### CounterManager API

- `add_counter(name, limit)`: Register a new counter with a limit
- `increment(name)`: Increment counter and return true if successful
- `can_increment(name)`: Check if counter can be incremented without actually incrementing
- `get_count(name)`: Get current count for a counter
- `get_limit(name)`: Get limit for a counter
- `reset(name)`: Reset a counter to 0

### Resetting Counters

Counters can be reset based on game events, time, or other conditions:

```rust
pub fn reset_on_event(mut counter_manager: ResMut<CounterManager>, /* event conditions */) {
    // Reset specific counters when certain events occur
    counter_manager.reset("effects");  // Reset effect counter
    counter_manager.reset("spawns");   // Reset spawn counter
}
```

### Advantages

- **Independent Limits**: Each behavior can have its own limit
- **Named Counters**: Easy to understand and maintain
- **Flexible**: Add/remove counters dynamically
- **Type-safe**: Compile-time checking of counter names

This approach is ideal when you need different iteration limits for different game mechanics or system behaviors.

## Examples

The `examples/` directory contains runnable examples demonstrating each system type. You can run them with:

```bash
cargo run --example run_once_example
cargo run --example iteration_limiter_example
cargo run --example limited_print_example
cargo run --example multi_counter_example
cargo run --example counter_users_example
cargo run --example counter_reset_example
```

### Using Systems in Your Own Code

Each example shows how to use the condition functions with Bevy's `.run_if()`:

```rust
use iteration_limitation::prelude::*;

// Run a system only once
app.add_systems(Update, my_system.run_if(should_run));

// Run a system up to a maximum number of times
app.add_systems(Update, my_system.run_if(should_continue_iterations));

// Check if specific counters can be incremented
app.add_systems(Update, my_system.run_if(can_increment_prints));
```

## Development

### Running the Demo

To see all systems in action:

```bash
cargo run
```

### Running Tests

```bash
cargo test
```

## Contributing

Contributions are welcome! Please feel free to submit issues and pull requests.

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### 💬 Questions, Ideas, or Feedback?

We use GitHub Discussions as our primary space for community interaction.

- Whether you're:

- Asking a question

- Proposing a feature

- Reporting a non-critical issue

- Sharing an idea or use case

Please post it in Discussions. We do not monitor email—this helps keep everything transparent and searchable for future contributors.
