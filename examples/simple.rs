use borealis::Application;
use legion::*;

fn main() {
    let mut world = World::default();
    world.push((1usize, false, 5.3f32));
    pollster::block_on(Application::run());    
}