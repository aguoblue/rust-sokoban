use std::fmt::format;

use hecs::{Entity, World};
use crate::components::*;

// ANCHOR: entities
//墙实体
pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_static("/images/wall.png"),
        Wall {},
        Immovable {},
    ))
}

//地板实体
pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable::new_static("/images/floor.png"),
    ))
}

//箱子实体
pub fn create_box(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            &format!("/images/box_{}_1.png", colour),
            &format!("/images/box_{}_2.png", colour),
        ]),
        Box { colour },
        Movable {},
    ))
}

//方框斑点组件
pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable::new_static(&format!("/images/box_spot_{}.png", colour)),
        BoxSpot { colour },
    ))
}

//玩家实体
pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable::new_animated(vec![
            "/images/player_1.png",
            "/images/player_2.png",
            "/images/player_3.png",
        ]),
        Player {},
        Movable {},
    ))
}

//游戏实体
pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((Gameplay::default(),))
}
// ANCHOR_END: entities


pub fn create_event_queue(world: &mut World) -> Entity {
    world.spawn((EventQueue::default(),))
}



pub fn create_audio_store(world: &mut World) -> Entity {
    world.spawn((AudioStore::default(),))
}

// ANCHOR: create_time
pub fn create_time(world: &mut World) -> Entity {
    world.spawn((Time::default(),))
}
// ANCHOR_END: create_time