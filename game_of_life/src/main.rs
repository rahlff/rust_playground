use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use std::time::Duration;

mod game;
mod cell;

fn main() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;
    const CELL_HEIGHT: u32 = 10;
    const CELL_WIDTH: u32 = 10;
    const CELL_COUNT_Y: usize = (HEIGHT/CELL_HEIGHT) as usize;
    const CELL_COUNT_X: usize = (WIDTH/CELL_WIDTH) as usize;
    const CELL_COUNT: usize = CELL_COUNT_X*CELL_COUNT_Y;

    let mut _game = game::Game::new(CELL_COUNT_Y, CELL_COUNT_X);
    
    // Poor mans gfx
    let mut rects: [Rect; CELL_COUNT] = [Rect::new(0,0,0,0);CELL_COUNT];
    let mut i = 0;
    for x in 0..CELL_COUNT_X {
        for y in 0..CELL_COUNT_Y {
            let x_pos: i32 = x as i32 * CELL_WIDTH as i32;
            let y_pos: i32 = y as i32 * CELL_HEIGHT as i32;

            rects[i] = Rect::new(x_pos, y_pos, CELL_WIDTH, CELL_HEIGHT);
            i = i+1;
        }
    }

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Game of Life", WIDTH, HEIGHT)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
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

        canvas.set_draw_color(Color::RGB(0,0,0));
        canvas.clear();

        _game.update();

        let mut i = 0;
        for x in 0..CELL_COUNT_X {
            for y in 0..CELL_COUNT_Y {
                match _game.get_cell(x, y) {
                    cell::Cell::Empty => {
                        canvas.set_draw_color(Color::RGB(0,0,0));
                    }
                    cell::Cell::Alive => {
                        canvas.set_draw_color(Color::RGB(0,0,255));
                    }
                    cell::Cell::Dying => {
                        canvas.set_draw_color(Color::RGB(250,0,20));
                    }
                }
                let _ = canvas.fill_rect(rects[i]);
                i += 1;
            }
        }

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        let _ = canvas.draw_rects(&rects);

        canvas.present();
        ::std::thread::sleep(Duration::new(0 /*sec*/ , 1_000_000_000u32 / 5 /* nano */));
    }

}
