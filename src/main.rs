extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();

    for event in event_pump.poll_iter() {
        
    }

    println!("Hello, world!");
}
