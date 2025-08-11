use bevy::ecs::{
    event::{BufferedEvent, EventReader},
    system::ResMut,
};

use crate::{create_event_with_dialog, person_resource::PersonResource};

#[derive(BufferedEvent, Clone)]
pub struct ChangeLocationEvent;

create_event_with_dialog!(
    ChangeLocation,
    ChangeLocationEvent,
    change_location,
    DialogConfig {
        title: "Location".to_string(),
        input_type: DialogInputType::TEXT,
        positive_action: "Change".to_string(),
        negative_action: "Cancel".to_string()
    }
);

fn change_location(
    mut person: ResMut<PersonResource>,
    mut event_reader: EventReader<ChangeLocationEventResult>,
) {
    for event in event_reader.read() {
        person.location = event.result.clone();
    }
}
