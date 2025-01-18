use crate::components::{BoxColour, Position};
use crate::entities::*;
use ggez::audio::Source;
use ggez::Context;
use hecs::World;

// ANCHOR: init
// Initialize the level
pub fn initialize_level(world: &mut World, context: &mut Context) {
    // const MAP: &str = "
    // N N W W W W W W
    // W W W . . . . W
    // W . . . B . . W
    // W . . . . . . W 
    // W . P . . . . W
    // W . . . . . . W
    // W . . S . . . W
    // W . . . . . . W
    // W W W W W W W W
    // ";
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W 
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
    W . . . . . . W
    W W W W W W W W
    ";

// 其中：
// . 是空白位置
// W 是墙
// P 是玩家
// B 是箱子
// S 是箱子放置点
// N 是空：用于地图的外边缘
    load_map(world, MAP.to_string());
    load_sounds(world, context);
}


pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };

            // Figure out what object we should create
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColour::Red);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Blue);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColour::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
// ANCHOR_END: init


// ANCHOR: load_sounds
pub fn load_sounds(world: &mut World, context: &mut Context) {
    let mut query = world.query::<&mut crate::components::AudioStore>();
    let audio_store = query.iter().next().unwrap().1;

    let sounds = ["correct", "incorrect", "wall"];

    for sound in sounds.iter() {
        let sound_name = sound.to_string();
        let sound_path = format!("/sounds/{}.wav", sound_name);
        let sound_source = Source::new(context, sound_path).expect("expected sound loaded");

        audio_store
            .sounds
            .insert(sound_name, Box::new(sound_source));
    }
}
// ANCHOR_END: load_sounds