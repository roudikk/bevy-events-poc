use bevy::ecs::event::BufferedEvent;

use crate::{create_event, person_resource::PersonResource};

#[derive(BufferedEvent, Clone)]
pub struct GrowOlderEvent;

create_event!(GrowOlder, GrowOlderEvent, grow_older);

fn grow_older(mut person: ResMut<PersonResource>, mut event_reader: EventReader<GrowOlderEvent>) {
    for _ in event_reader.read() {
        person.age += 1;
    }
}
