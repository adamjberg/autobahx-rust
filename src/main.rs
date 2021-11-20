extern crate sdl2;

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;


pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("autobahx", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut player_pos: (i16, i16) = (0, 0);
    
    let mut keys_map = HashMap::new();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode,
                    ..
                } => {
                    match keycode {
                        Some(keycode) => {
                            keys_map.insert(keycode, true);
                        }
                        _ => {}
                    }
                }
                Event::KeyUp {
                    keycode,
                    ..
                } => {
                    match keycode {
                        Some(keycode) => {
                            keys_map.insert(keycode, false);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        const X_SPEED: i16 = 10;

        let mut x_direction = 0;
        if keys_map.get(&Keycode::Right) == Some(&true) {
            x_direction = 1;
        } else if keys_map.get(&Keycode::Left) == Some(&true) {
            x_direction = -1;
        }
        player_pos.0 = player_pos.0 + (X_SPEED * x_direction);
        
        const Y_SPEED: i16 = 10;
        let mut y_direction = 0;
        if keys_map.get(&Keycode::Up) == Some(&true) {
            y_direction = -1;
        } else if keys_map.get(&Keycode::Down) == Some(&true) {
            y_direction = 1;
        }
        player_pos.1 = player_pos.1 + (Y_SPEED * y_direction);

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        

        canvas.box_(player_pos.0, player_pos.1, player_pos.0 + 30, player_pos.1 + 100, Color::WHITE)?;
        // canvas.set_draw_color(Color::WHITE);
        

        canvas.present();
    }

    Ok(())
}