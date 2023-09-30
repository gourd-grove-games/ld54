use crate::nutrients::Nutrient;
use bevy::prelude::*;

#[derive(Component)]
pub struct Land;

#[derive(Component)]
pub struct GridPos {
    pub x: i32,
    pub y: i32,
}
