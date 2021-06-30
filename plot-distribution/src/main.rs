extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;
use rand::prelude::*;
 

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    const BAR_WIDTH: u32 = 10;
    const STEP: u32 = 5;
    const START_HEIGHT: u32 = 20;
 
    let window = video_subsystem.window("rust-sdl2 demo", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();

    let mut rects: [Rect; 80] = [Rect::new(0,0,0,0);80];

    for i in 0..rects.len() {
        let x:i32 = i as i32*BAR_WIDTH as i32;
        let y:i32 = (HEIGHT as i32)-20;

        //                   x, y,      w,        h
        rects[i] = Rect::new(x, y, BAR_WIDTH, START_HEIGHT);
    }
 
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        let secret_number = rand::thread_rng().gen_range(0..80);

        rects[secret_number].set_height(rects[secret_number].height() + STEP);
        rects[secret_number].set_y((HEIGHT as i32) - (rects[secret_number].height() as i32));

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.draw_rects(&rects);

        canvas.present();
        ::std::thread::sleep(Duration::new(0 /*sec*/ , 1_000_000_000u32 / 180 /* nano */));
    }
}
