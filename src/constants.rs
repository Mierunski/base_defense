use bevy::prelude::Color;

pub const RESOLUTION: f32 = 16.0 / 9.0;
pub const TILE_SIZE: f32 = 0.15;

pub const MAP_SIZE: i32 = 100;
// Bound size used by noise generation
pub const BOUND_SIZE: f64 = 8.0;

pub const PROJECTILE_LAYER: f32 = 20.0;
pub const STARTING_GOLD: f32 = 100.0;

pub const COLOR_TOWER: Color = Color::rgb(0.8, 0.2, 0.2);
pub const COLOR_ENEMY: Color = Color::rgb(0.2, 0.8, 0.2);
pub const COLOR_MINER: Color = Color::rgb(0.3, 0.2, 0.5);
