use ecs::World;
use ecs::BuildData;

//Generally, you should use #[cold] by default, and #[hot] for the most important components that are accessed a lot and used by all
components! {
    struct MyComponents {
        #[hot] position: Position,
        #[cold] respawn: Position
    }
}

systems! {
    struct MySystems<MyComponents, ()>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

pub fn initialize_ecs() {
    let mut world = World::<MySystems>::new();
    let entity = world.create_entity(());

    let entity = world.create_entity(
    |entity: BuildData<MyComponents>, data: &mut MyComponents| {
        data.position.add(&entity, Position { x: 0.0, y: 0.0 });
        data.respawn.add(&entity, Position { x: 0.0, y: 0.0 });
    }
);
}