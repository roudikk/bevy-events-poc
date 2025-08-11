use bevy::ecs::event::BufferedEvent;
use rand::Rng;

use crate::{create_event_with_dialog, person_resource::PersonResource};

#[derive(BufferedEvent, Clone)]
pub struct ChangeCounterEvent {
    pub counter: u32,
}

create_event_with_dialog!(
    ChangeCounter,
    ChangeCounterEvent,
    change_counter,
    DialogConfig {
        title: "This will be added to some random".to_string(),
        input_type: DialogInputType::NUMBER,
        positive_action: "Randomize!".to_string(),
        negative_action: "Cancel".to_string()
    }
);

fn change_counter(
    mut person: ResMut<PersonResource>,
    mut event_reader: EventReader<ChangeCounterEventResult>,
) {
    for event in event_reader.read() {
        let mut rng = rand::thread_rng();
        let random_number: u32 = rng.gen_range(1..100) + event.result.parse::<u32>().unwrap();
        let initial = event.event.counter;
        person.counter = initial + random_number;
    }
}
