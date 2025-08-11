use crate::{create_event, person_resource::PersonResource};
use bevy_egui::{
    EguiContexts,
    egui::{self, Color32, Frame, Margin, Pos2, RichText, Stroke},
};

create_event!(
    OpenColorPicker,
    OpenColorPickerEvent,
    open_color_picker,
    color_picker_window
);

#[derive(BufferedEvent, Clone)]
pub struct OpenColorPickerEvent {
    pub color: Color,
}

#[derive(Component)]
struct ColorPicker {
    title: String,
    color: Color,
}

fn open_color_picker(
    mut commands: Commands,
    mut reader: EventReader<OpenColorPickerEvent>,
    pickers: Query<(Entity, &ColorPicker)>,
) {
    for event in reader.read() {
        for (entity, _) in pickers {
            commands.entity(entity).despawn();
        }

        commands.spawn(ColorPicker {
            title: "Change Color".to_string(),
            color: event.color.clone(),
        });
    }
}

fn color_picker_window(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut color_picker: Query<(Entity, &mut ColorPicker)>,
    mut person: ResMut<PersonResource>,
    window: Single<&Window>,
) {
    if color_picker.is_empty() {
        return;
    }

    let (entity, mut color_picker) = color_picker.single_mut().unwrap();

    let mut is_open = true;
    egui::Window::new("")
        .open(&mut is_open)
        .interactable(true)
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .order(egui::Order::Foreground)
        .default_pos(Pos2::new(
            window.width() / 2.0 - 450.0 / 2.0,
            window.height() / 2.0 - 500.0 / 2.0,
        ))
        .show(contexts.ctx_mut().unwrap(), |ui| {
            render_color_picker_content(ui, &mut color_picker, &mut person, &mut commands, entity);
        });

    if !is_open {
        commands.entity(entity).despawn();
    }
}

fn render_color_picker_content(
    ui: &mut egui::Ui,
    color_picker: &mut ColorPicker,
    person: &mut PersonResource,
    commands: &mut Commands,
    entity: Entity,
) {
    ui.set_min_width(450.0);
    ui.set_max_width(450.0);

    Frame::new().show(ui, |ui| {
        ui.label(
            egui::RichText::new(color_picker.title.clone())
                .size(18.0)
                .strong()
                .color(Color32::WHITE),
        );

        ui.add_space(12.0);

        render_enhanced_color_bands(ui, color_picker);

        ui.add_space(24.0);

        Frame::new()
            .fill(Color32::from_rgba_unmultiplied(20, 18, 25, 120))
            .corner_radius(12.0)
            .inner_margin(Margin::symmetric(8, 6))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Save").color(Color32::from_rgb(255, 255, 255)),
                                )
                                .fill(Color32::from_rgb(70, 140, 70))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(90, 160, 90)))
                                .corner_radius(10.0)
                                .min_size(egui::Vec2::new(120.0, 36.0)),
                            )
                            .clicked()
                        {
                            person.color = color_picker.color.clone();
                            commands.entity(entity).despawn();
                        }

                        ui.add_space(12.0);

                        if ui
                            .add(
                                egui::Button::new(
                                    RichText::new("Cancel").color(Color32::from_rgb(255, 255, 255)),
                                )
                                .fill(Color32::from_rgb(200, 50, 70))
                                .stroke(Stroke::new(1.0, Color32::from_rgb(90, 90, 90)))
                                .corner_radius(10.0)
                                .min_size(egui::Vec2::new(80.0, 36.0)),
                            )
                            .clicked()
                        {
                            commands.entity(entity).despawn();
                        }
                    });
                });
            });
    });
}

fn render_enhanced_color_bands(ui: &mut egui::Ui, color_picker: &mut ColorPicker) {
    let mut color_arrays = convert_color_f32(&color_picker.color);

    render_color_bands(ui, "What is your color?", &mut color_arrays, "Who you is~");

    color_picker.color = convert_arrays_to_colors(&color_arrays);
}

fn render_color_bands(ui: &mut egui::Ui, label: &str, colors: &mut [f32; 4], description: &str) {
    ui.push_id(format!("color_band_{label}"), |ui| {
        Frame::new()
            .fill(Color32::from_rgba_unmultiplied(18, 16, 22, 200))
            .stroke(Stroke::new(
                1.0,
                Color32::from_rgba_unmultiplied(45, 40, 50, 180),
            ))
            .corner_radius(12.0)
            .inner_margin(Margin::same(18))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Enhanced labels
                    ui.label(
                        egui::RichText::new(label)
                            .size(16.0)
                            .strong()
                            .color(Color32::WHITE),
                    );

                    ui.label(
                        egui::RichText::new(description)
                            .size(12.0)
                            .color(Color32::from_rgb(180, 180, 180)),
                    );

                    ui.add_space(8.0);

                    // Color controls with better layout
                    ui.horizontal(|ui| {
                        // Color picker button with custom styling
                        ui.color_edit_button_rgba_unmultiplied(colors);

                        ui.add_space(12.0);

                        // Hex/RGB display with monospace font
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            Frame::new()
                                .fill(Color32::from_rgba_unmultiplied(30, 28, 35, 180))
                                .corner_radius(6.0)
                                .inner_margin(Margin::symmetric(8, 4))
                                .show(ui, |ui| {
                                    ui.label(
                                        egui::RichText::new(format!(
                                            "#{:02X}{:02X}{:02X}",
                                            (colors[0] * 255.0) as u8,
                                            (colors[1] * 255.0) as u8,
                                            (colors[2] * 255.0) as u8
                                        ))
                                        .size(11.0)
                                        .family(egui::FontFamily::Monospace)
                                        .color(Color32::from_rgb(200, 200, 200)),
                                    );
                                });
                        });
                    });
                });
            });
    });
}

fn convert_color_f32(color: &Color) -> [f32; 4] {
    color.to_linear().to_f32_array()
}

fn convert_arrays_to_colors(arrays: &[f32; 4]) -> Color {
    Color::linear_rgba(arrays[0], arrays[1], arrays[2], arrays[3])
}
