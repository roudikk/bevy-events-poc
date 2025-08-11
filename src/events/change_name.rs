use bevy::ecs::{
    event::{BufferedEvent, EventReader},
    system::ResMut,
};

use crate::{create_event_with_dialog, person_resource::PersonResource};

#[derive(BufferedEvent, Clone)]
pub struct ChangeNameEvent;

create_event_with_dialog!(
    ChangeName,
    ChangeNameEvent,
    change_name,
    DialogConfig {
        title: "Name".to_string(),
        input_type: DialogInputType::TEXT,
        positive_action: "Change".to_string(),
        negative_action: "Cancel".to_string()
    }
);

fn change_name(
    mut person: ResMut<PersonResource>,
    mut event_reader: EventReader<ChangeNameEventResult>,
) {
    for event in event_reader.read() {
        person.name = event.result.clone();
    }
}
