use std::fmt::format;

use hecs::{Entity, World};
use crate::components::*;

// ANCHOR: entities
//墙实体
pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
        Immovable{},
    ))
}

//地板实体
pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

//箱子实体
pub fn create_box(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: format!("/images/box_{}.png", colour),
        },
        Box {colour},
        Movable{},
    ))
}

//方框斑点组件
pub fn create_box_spot(world: &mut World, position: Position, colour: BoxColour) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: format!("/images/box_spot_{}.png", colour),
        },
        BoxSpot {colour},
    ))
}

//玩家实体
pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
        Movable{},
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