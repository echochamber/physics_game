use ecs::World;
use ecs::BuildData;
use ecs::System;
use ecs::Process;
use ecs::DataHelper;
use ecs::ModifyData;
use ecs::EntityData;

//Generally, you should use #[cold] by default, and #[hot] for the most important components that are accessed a lot and used by all
components! {
    struct MyComponents {
        #[hot] position: Position,
        #[cold] respawn: Position
    }
}

systems! {
    struct MySystems<MyComponents, ()> {
        print_msg: PrintMessage = PrintMessage("Hello World".to_string())
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

// System from ECS
pub struct PrintMessage(pub String);

impl System for PrintMessage {
    type Components = MyComponents;
    type Services = ();
}

impl Process for PrintMessage {
    fn process(&mut self, _: &mut DataHelper<MyComponents, ()>) {
        println!("{}", &self.0);
    }
}

pub fn initialize_ecs() {
    let mut world = World::<MySystems>::new();
    let entity = world.create_entity(()); // Create an entity without any components, passing it a closure that returns unit ()

    let entity = world.create_entity(
        |entity: BuildData<MyComponents>, data: &mut MyComponents| { // Create an entity with components
            data.position.add(&entity, Position { x: 0.0, y: 0.0 }); // Add a position componenet to the entity
            data.respawn.add(&entity, Position { x: 0.0, y: 0.0 }); //  Add a respawn component to the entity
        }
    );

    // Edit the position component of an entity
    world.with_entity_data(&entity, |entity, data| {
        data.position[entity].x += 5.0;
        data.position[entity].y += 8.0;
    });

    // Edit what componenets an entity has
    world.modify_entity(entity,
        |entity: ModifyData<MyComponents>, data: &mut MyComponents| {
            data.respawn[entity].x -= 4.0;
            data.position[entity] = data.respawn[entity];
            data.respawn.remove(&entity);
            assert_eq!(data.respawn.get(&entity), None);
            data.respawn.insert(&entity, Position { x: 1.0, y: 2.0});
        }
    );

    world.update();
}