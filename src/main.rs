#![feature(macro_metavar_expr_concat)]

use bevy::{
    DefaultPlugins,
    app::{App, Startup},
    camera::Camera2d,
    ecs::{
        event::EventWriter,
        system::{Commands, Res},
    },
};
use bevy_egui::{
    EguiContexts, EguiPlugin, EguiPrimaryContextPass,
    egui::{self, Align2, Id},
};

use crate::{
    events::{
        change_color::OpenColorPickerEvent,
        change_counter::ChangeCounterEvent,
        change_location::ChangeLocationEvent,
        change_name::ChangeNameEvent,
        events::{AppEvent, EventsPlugin},
        grow_older::GrowOlderEvent,
    },
    person_resource::PersonResource,
};

mod events;
mod person_resource;

fn main() {
    App::new()
        .insert_resource(PersonResource::default())
        .add_plugins((DefaultPlugins, EventsPlugin, EguiPlugin::default()))
        .add_systems(Startup, startup)
        .add_systems(EguiPrimaryContextPass, render_ui)
        .run();
}

fn startup(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn render_ui(
    mut context: EguiContexts,
    mut event_writer: EventWriter<AppEvent>,
    person: Res<PersonResource>,
) {
    let ctx = context.ctx_mut().unwrap();
    egui::Window::new("Person")
        .id(Id::new("Person Window"))
        .anchor(Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
        .fixed_size(egui::Vec2::new(500.0, 500.0))
        .show(ctx, |ui| {
            ui.heading("Person");
            ui.add_space(16.0);

            ui.label(format!("Name: {}", &person.name));
            if ui.button("Change").clicked() {
                event_writer.write(AppEvent::ChangeName(ChangeNameEvent));
            }
            ui.add_space(8.0);

            ui.label(format!("Age: {}", person.age.to_string()));
            if ui.button("Grow old").clicked() {
                event_writer.write(AppEvent::GrowOlder(GrowOlderEvent));
            }
            ui.add_space(8.0);

            ui.label(format!("Location: {}", &person.location));
            if ui.button("Change").clicked() {
                event_writer.write(AppEvent::ChangeLocation(ChangeLocationEvent));
            }
            ui.add_space(8.0);

            ui.label(format!("Color: {:?}", &person.color));
            if ui.button("Change").clicked() {
                event_writer.write(AppEvent::OpenColorPicker(OpenColorPickerEvent {
                    color: person.color.clone(),
                }));
            }
            ui.add_space(8.0);

            ui.label(format!("Counter: {}", &person.counter.to_string()));
            if ui.button("Randomize").clicked() {
                event_writer.write(AppEvent::ChangeCounter(ChangeCounterEvent {
                    counter: person.counter,
                }));
            }
            ui.add_space(8.0);
        });
}
