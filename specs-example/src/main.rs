extern crate specs;
// #[macro_use]
// extern crate specs_derive;

#[derive(Debug)] // Component
//#[storage(VecStorage)]
struct Position {
    x: f32,
    y: f32
}

impl specs::Component for Position {
    type Storage = specs::VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: f32,
    y: f32
}

impl specs::Component for Velocity {
    type Storage = specs::VecStorage<Self>;
}

struct HelloWorld;

impl<'a> specs::System<'a> for HelloWorld {
    type SystemData = specs::ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for position in position.join() {
            println!("Hello, {:?}", &position);
        }
    }
}

fn main() {
    use specs::Builder;
    use specs::RunNow;

    let mut world = specs::World::new();

    // This will create a component storage for Position
    world.register::<Position>();

    let ball = world.create_entity()
        .with(Position {
            x: 4.0,
            y: 7.0
        })
        .build();

    let mut hello_world = HelloWorld;
    hello_world.run_now(&world.res);
    world.maintain();
}
