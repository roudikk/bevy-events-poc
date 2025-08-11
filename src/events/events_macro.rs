#![allow(dead_code)]
#![allow(non_snake_case)]

#[macro_export]
macro_rules! create_event_with_dialog {
    ($plugin_name:ident, $event_type:ident, $handler_fn:ident, $config:expr) => {
        use bevy::prelude::*;
        use bevy_egui::{
            EguiPrimaryContextPass,
            EguiContexts,
            egui::{self, Frame, Color32, RichText, Vec2},
        };

        pub enum DialogInputType {
            NUMBER,
            TEXT,
        }

        pub struct DialogConfig {
            title: String,
            input_type: DialogInputType,
            positive_action: String,
            negative_action: String,
        }

        #[derive(BufferedEvent)]
        pub struct ${concat($event_type, Result)} {
            event: $event_type,
            result: String
        }

        pub struct ${concat($plugin_name, Plugin)};

        #[derive(Component)]
        pub struct ${concat($event_type, Window)} {
            event: $event_type,
            input: String,
        }

        impl Plugin for ${concat($plugin_name, Plugin)} {
            fn build(&self, app: &mut App) {
                app
                    .add_event::<${concat($event_type, Result)}>()
                    .add_event::<$event_type>()
                    .add_systems(
                        Update,
                        open_dialog.run_if(has_open_dialog_events)
                    )
                    .add_systems(
                        Update,
                        $handler_fn.run_if(has_result_events)
                    )
                    .add_systems(
                        EguiPrimaryContextPass,
                        dialog_window.run_if(window_active)
                    );
            }
        }

        fn window_active(
            query: Query<Entity, With<${concat($event_type, Window)}>>,
        ) -> bool {
            !query.is_empty()
        }

        fn has_result_events(
            event_reader: EventReader<${concat($event_type, Result)}>
        ) -> bool {
            !event_reader.is_empty()
        }

        fn has_open_dialog_events(
            event_reader: EventReader<$event_type>
        ) -> bool {
            !event_reader.is_empty()
        }

        fn open_dialog(
            mut commands: Commands,
            mut event_reader: EventReader<$event_type>,
            query: Query<Entity, With<${concat($event_type, Window)}>>,
        ) {
            for event in event_reader.read() {
                for entity in query.iter() {
                    commands.entity(entity).despawn();
                }
                commands.spawn(${concat($event_type, Window)} {
                    event: event.to_owned(),
                    input: "".to_string(),
                });
            }
        }

        fn dialog_window(
            mut commands: Commands,
            mut window: Single<(Entity, &mut ${concat($event_type, Window)})>,
            mut contexts: EguiContexts,
            mut event_writer: EventWriter<${concat($event_type, Result)}>
        ) {
            let ctx = contexts.ctx_mut().unwrap();
            let config: DialogConfig = $config;

            let mut open = true;
            egui::Window::new(config.title.clone())
                .open(&mut open)
                .interactable(true)
                .collapsible(false)
                .resizable(false)
                .title_bar(false)
                .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
                .show(ctx, |ui| {
                    Frame::new().show(ui, |ui| {
                        ui.heading(config.title);
                        ui.add_space(16.0);

                        let mut new_input = window.1.input.clone();
                        ui.text_edit_singleline(&mut new_input);

                        let new_input = match config.input_type {
                            DialogInputType::NUMBER => new_input.chars()
                                .filter(|c| c.is_ascii_digit() || *c == '.' || *c == '-')
                                .collect(),
                            DialogInputType::TEXT => new_input,
                        };

                        ui.add_space(16.0);

                        window.1.input = new_input.to_string();

                        ui.horizontal(|ui| {
                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(config.positive_action).color(Color32::from_rgb(255, 255, 255)),
                                    )
                                    .fill(Color32::from_rgb(70, 140, 70))
                                    .corner_radius(10.0)
                                    .min_size(egui::Vec2::new(120.0, 36.0)),
                                )
                                .clicked()
                                && !new_input.is_empty()
                            {
                                event_writer.write(${concat($event_type, Result)} {
                                    event: window.1.event.clone(),
                                    result: new_input
                                });
                                commands.entity(window.0).despawn();
                            }

                            ui.add_space(12.0);

                            if ui
                                .add(
                                    egui::Button::new(
                                        RichText::new(config.negative_action).color(Color32::from_rgb(255, 255, 255)),
                                    )
                                    .fill(Color32::from_rgb(200, 50, 70))
                                    .corner_radius(10.0)
                                    .min_size(egui::Vec2::new(120.0, 36.0)),
                                )
                                .clicked()
                            {
                                commands.entity(window.0).despawn();
                            }
                        });
                    });
                });

            if !open {
                commands.entity(window.0).despawn();
            }
        }
    };
}

#[macro_export]
macro_rules! create_event {
    ($plugin_name:ident, $event_type:ty, $handler_fn:ident) => {
        use bevy::prelude::*;

        pub struct ${concat($plugin_name, Plugin)};

        impl Plugin for ${concat($plugin_name, Plugin)} {
            fn build(&self, app: &mut App) {
                app.add_event::<$event_type>().add_systems(
                    bevy::app::Update,
                    $handler_fn.run_if(|reader: EventReader<$event_type>| !reader.is_empty()),
                );
            }
        }
    };

    ($plugin_name:ident, $event_type:ty, $handler_fn:expr, $ui_system:expr) => {
        use bevy::prelude::*;
        use bevy_egui::EguiPrimaryContextPass;

        pub struct ${concat($plugin_name, Plugin)};

        impl Plugin for ${concat($plugin_name, Plugin)} {
            fn build(&self, app: &mut App) {
                app.add_event::<$event_type>()
                    .add_systems(
                        bevy::app::Update,
                        $handler_fn.run_if(|reader: EventReader<$event_type>| !reader.is_empty()),
                    )
                    .add_systems(EguiPrimaryContextPass, $ui_system);
            }
        }
    };
}

#[macro_export]
macro_rules! create_events_plugin {
    ($plugin_name:ident, $($event_plugin:ident),+ $(,)?) => {
        use bevy::prelude::*;

        #[derive(BufferedEvent)]
        pub enum AppEvent {
            $(
                $event_plugin(${concat($event_plugin, Event)}),
            )+
        }

        pub struct $plugin_name;

        impl Plugin for $plugin_name {
            fn build(&self, app: &mut App) {
                app
                    .add_event::<AppEvent>()
                    .add_systems(Update, on_track_event.run_if(has_track_events))
                    .add_plugins(($(${concat($event_plugin, Plugin)},)+));
            }
        }

        fn has_track_events(
            event_reader: EventReader<AppEvent>
        ) -> bool {
            !event_reader.is_empty()
        }

        #[allow(non_snake_case)]
        fn on_track_event(
            mut event_reader: EventReader<AppEvent>,
            $(
                mut ${concat($event_plugin, Writer)}: EventWriter<${concat($event_plugin, Event)}>,
            )+
        ) {
            for event in event_reader.read() {
                match event {
                    $(
                        AppEvent::$event_plugin(e) => {
                            ${concat($event_plugin, Writer)}.write(e.clone());
                        }
                    )+
                }
            }
        }
    };
}
