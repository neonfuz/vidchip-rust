extern crate sdl2;

use sdl2::event::Event;
use sdl2::pixels::Color;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video.window("chip", 320, 240).build().unwrap();
    let mut renderer = window.renderer()
        .present_vsync().build().unwrap();

    let mut time : u8 = 0;
    let mut event_pump = sdl.event_pump().unwrap();
    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => { break 'main; }
                _ => {}
            }
        }
        time = time.overflowing_add(1).0;
        renderer.set_draw_color(Color::RGB(time, time, time));
        renderer.clear();
        renderer.present();
    }
}
