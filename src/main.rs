extern crate sdl2;

use sdl2::event::Event;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();
    let video = sdl.video().unwrap();

    let window = video.window("chip", 320, 240).build().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'main; }
                _ => {}
            }
        }
        

    }

    println!("Hello, world!");
}
