use bevy::color::Color;
use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct PersonResource {
    pub name: String,
    pub age: u32,
    pub location: String,
    pub color: Color,
    pub counter: u32,
}
