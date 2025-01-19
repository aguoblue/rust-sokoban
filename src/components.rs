use std::fmt;
use std::fmt::Display;
use crate::events::Event;
use ggez::audio;
use ggez::audio::SoundSource;
use ggez::Context;
use std::collections::HashMap;
use std::time::Duration;

// ANCHOR: components
#[derive(Clone, Copy)]
pub struct Position {
    pub x: u8,
    pub y: u8,
    pub z: u8,
}

// ANCHOR: renderable
pub struct Renderable {
    paths: Vec<String>,
}
// ANCHOR_END: renderable

// ANCHOR: renderable_kind
pub enum RenderableKind {
    Static,
    Animated,
}
// ANCHOR_END: renderable_kind


// ANCHOR: renderable_impl
impl Renderable {
    // ANCHOR: renderable_new_fn
    pub fn new_static(path: &str) -> Self {
        Self {
            paths: vec![path.to_string()],
        }
    }

    pub fn new_animated(paths: Vec<&str>) -> Self {
        Self {
            paths: paths.iter().map(|p| p.to_string()).collect(),
        }
    }
    // ANCHOR_END: renderable_new_fn
    // ANCHOR_END: renderable_impl

    // ANCHOR: renderable_kind_fn
    pub fn kind(&self) -> RenderableKind {
        match self.paths.len() {
            0 => panic!("invalid renderable"),
            1 => RenderableKind::Static,
            _ => RenderableKind::Animated,
        }
    }
    // ANCHOR_END: renderable_kind_fn

    // ANCHOR: renderable_path_fn
    pub fn path(&self, path_index: usize) -> String {
        // If we get asked for a path that is larger than the
        // number of paths we actually have, we simply mod the index
        // with the length to get an index that is in range.
        self.paths[path_index % self.paths.len()].clone()
    }
    // ANCHOR_END: renderable_path_fn
}


pub struct Wall {}

pub struct Player {}

pub struct Box {
    pub colour: BoxColour,
}

pub struct BoxSpot {
    pub colour: BoxColour,
}

pub struct Movable;

pub struct Immovable;

// ANCHOR_END: components


// ANCHOR: gameplay_state
#[derive(Default)]
pub enum GameplayState {
    #[default]
    Playing,
    Won,
}

#[derive(Default)]
pub struct Gameplay {
    pub state: GameplayState,
    pub moves_count: u32,
}
// ANCHOR_END: gameplay_state

// ANCHOR: gameplay_state_impl_default
// ANCHOR_END: gameplay_state_impl_default

// ANCHOR: gameplay_state_impl_display
impl Display for GameplayState {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            GameplayState::Playing => "Playing",
            GameplayState::Won => "Won",
        })?;
        Ok(())
    }
}
// ANCHOR_END: gameplay_state_impl_display



// ANCHOR: box_colour_eq
#[derive(PartialEq)]
// ANCHOR: box_colour
pub enum BoxColour {
    Red,
    Blue,
}
// ANCHOR_END: box_colour
// ANCHOR_END: box_colour_eq

// ANCHOR: box_colour_display
impl Display for BoxColour {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(match self {
            BoxColour::Red => "red",
            BoxColour::Blue => "blue",
        })?;
        Ok(())
    }
}
// ANCHOR_END: box_colour_display


// ANCHOR: events
#[derive(Default)]
pub struct EventQueue {
    pub events: Vec<Event>,
}
// ANCHOR_END: events


// ANCHOR: audio_store
#[derive(Default)]
pub struct AudioStore {
    pub sounds: HashMap<String, std::boxed::Box<audio::Source>>,
}
// ANCHOR_END: audio_store

// ANCHOR: audio_store_impl
impl AudioStore {
    pub fn play_sound(&mut self, context: &mut Context, sound: &str) {
        if let Some(source) = self.sounds.get_mut(sound) {
            if source.play_detached(context).is_ok() {
                println!("Playing sound: {}", sound);
            }
        }
    }
}
// ANCHOR_END: audio_store_impl

// ANCHOR: create_time
#[derive(Default)]
pub struct Time {
    pub delta: Duration,
}
// ANCHOR_END: create_time