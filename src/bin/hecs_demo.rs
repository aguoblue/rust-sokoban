use hecs::World;

/// 定义组件：位置
#[derive(Debug)]
struct Position {
    x: f32,
    y: f32,
}

/// 定义组件：速度
#[derive(Debug)]
struct Velocity {
    dx: f32,
    dy: f32,
}

fn main() {
    // 创建一个 ECS 世界
    let mut world = World::new();

    // 创建实体并添加组件
    let _entity1 = world.spawn((Position { x: 0.0, y: 0.0 }, Velocity { dx: 1.0, dy: 1.0 }));
    let _entity2 = world.spawn((Position { x: 5.0, y: 5.0 }, Velocity { dx: -0.5, dy: 0.5 }));
    let _entity3 = world.spawn((Position { x: 10.0, y: 10.0 },)); // 只有位置组件

    // 模拟循环
    for _ in 0..5 {
        println!("Before update:");
        for (id, pos) in world.query::<&Position>().iter() {
            println!("Entity {:?} - Position: {:?}", id, pos);
        }

        // 更新系统：根据速度更新位置
        for (_, (pos, vel)) in world.query::<(&mut Position, &Velocity)>().iter() {
            pos.x += vel.dx;
            pos.y += vel.dy;
        }

        println!("\nAfter update:");
        for (id, pos) in world.query::<&Position>().iter() {
            println!("Entity {:?} - Position: {:?}", id, pos);
        }

        println!("====================");
    }
}
