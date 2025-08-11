# Bevy Events Proof of Concept

This project implements a proof-of-concept for a modular event system in Bevy that automatically generates UI dialogs and handles user interactions through a macro-driven architecture. The application presents a simple "Person" interface where users can modify various attributes through different types of event-driven interactions.

## Features

- **Macro-Based Event System**: Custom macros that automatically generate event plugins, UI dialogs, and event handlers
- **Multiple Event Types**: Demonstrates different patterns for handling events with and without UI dialogs
- **Resource Management**: Clean separation between UI state and application data
- **Type Safety**: Compile-time event routing and type checking

## Architecture

### Event System Components

1. **`create_events_plugin!`**: Creates a central event router that dispatches events to individual handlers
2. **`create_event!`**: Creates simple events that execute immediately without UI interaction
3. **`create_event_with_dialog!`**: Generates events that open UI dialogs for user input

### Event Types Demonstrated

- **Name Change**: Text input dialog for changing the person's name
- **Age**: Simple button click to increment age (no dialog)
- **Location Change**: Text input dialog for changing location
- **Color Picker**: Color Selection Dialog
- **Counter**: Random number generation with current value context

### Macro System

The macro system automatically generates:

- Event structs with proper Bevy event derivations
- Plugin structs implementing Bevy's Plugin trait
- UI systems for dialog rendering
- Event handling systems with proper run conditions
- Type-safe event routing and dispatch

### Adding New Events

To add a new event type:

1. Create a new file in `src/events/`
2. Define your event struct
3. Choose the appropriate macro (`create_event_with_dialog!` or `create_event!`)
4. Implement the event handler function
5. Register the event in `src/events/events.rs`

### Example Event Implementation

```rust
use crate::{create_event_with_dialog, person_resource::PersonResource};

#[derive(BufferedEvent, Clone)]
pub struct MyCustomEvent;

create_event_with_dialog!(
    MyCustom,
    MyCustomEvent,
    handle_my_event,
    DialogConfig {
        title: "My Dialog".to_string(),
        input_type: DialogInputType::TEXT,
        positive_action: "Apply".to_string(),
        negative_action: "Cancel".to_string()
    }
);

fn handle_my_event(
    mut person: ResMut<PersonResource>,
    mut event_reader: EventReader<MyCustomEventResult>,
) {
    for event in event_reader.read() {
        // Handle the event result
    }
}
```
