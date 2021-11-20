extern crate sdl2;

use std::collections::HashMap;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::gfx::primitives::DrawRenderer;

const STAGE_WIDTH: i16 = 800;
const STAGE_HEIGHT: i16 = 600;
const NUM_LANES: i16 = 5;
const LANE_WIDTH: i16 = 40; 

enum GameState {
    Paused,
    Playing,
    GameOver,
}

struct TrafficCar {
    y: i16,
    lane: i16
}

fn get_random_traffic_car() -> TrafficCar {
    let y_rand = (rand::random::<f64>() * -400.0) as i16 - 400;
    let lane_rand = (rand::random::<f64>() * 5.0) as i16;

    return TrafficCar {
        y: y_rand,
        lane: lane_rand
    }
}

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("autobahx", STAGE_WIDTH.try_into().unwrap(), STAGE_HEIGHT.try_into().unwrap())
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut game_state = GameState::Playing;

    let player_size: (i16, i16) = (30, 60);
    let mut player_pos: (i16, i16) = (STAGE_WIDTH / 2 - player_size.0 / 2, STAGE_HEIGHT - player_size.1);

    let mut traffic_cars: Vec<TrafficCar> = Vec::new();
    for i in 0..8 {
        traffic_cars.push(get_random_traffic_car())
    }

    let total_lane_width = NUM_LANES * LANE_WIDTH;
    let half_lane_width = total_lane_width / 2;
    let left_boundary = STAGE_WIDTH / 2 - half_lane_width;
    let right_boundary = STAGE_WIDTH / 2 + half_lane_width;

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
                            if keycode == Keycode::P {
                                match game_state {
                                    GameState::Paused => {
                                        game_state = GameState::Playing;
                                    }
                                    GameState::Playing => {
                                        game_state = GameState::Paused;
                                    }
                                    _ => {}
                                }
                            }

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

        match game_state {
            GameState::Paused => {
                continue;
            }
            _ => {}
        }

        const X_SPEED: i16 = 2;

        let mut x_direction = 0;
        if keys_map.get(&Keycode::Right) == Some(&true) {
            x_direction = 1;
        } else if keys_map.get(&Keycode::Left) == Some(&true) {
            x_direction = -1;
        }
        player_pos.0 = player_pos.0 + (X_SPEED * x_direction);
        if player_pos.0 < left_boundary {
            player_pos.0 = left_boundary;
        } else if player_pos.0 + player_size.0 > right_boundary {
            player_pos.0 = right_boundary - player_size.0;
        }
        
        const Y_SPEED: i16 = 2;
        let mut y_direction = 0;
        if keys_map.get(&Keycode::Up) == Some(&true) {
            y_direction = -1;
        } else if keys_map.get(&Keycode::Down) == Some(&true) {
            y_direction = 1;
        }
        player_pos.1 = player_pos.1 + (Y_SPEED * y_direction);
        if player_pos.1 < 0 {
            player_pos.1 = 0;
        } else if player_pos.1 + player_size.1 > STAGE_HEIGHT {
            player_pos.1 = STAGE_HEIGHT - player_size.1;
        }

        for car in traffic_cars.iter_mut() {
            car.y += 2;

            if car.y > STAGE_HEIGHT {
                let rand_car = get_random_traffic_car();
                car.lane = rand_car.lane;
                car.y = rand_car.y;
            }

            let car_left = left_boundary + car.lane * LANE_WIDTH;
            let car_right = car_left + player_size.0;
            let car_top = car.y;
            let car_bottom = car.y + player_size.1;

            if player_pos.0 + player_size.0 > car_left && player_pos.0 < car_right && player_pos.1 + player_size.1 > car_top && player_pos.1 < car_bottom {
                game_state = GameState::GameOver;
                break 'running;
            }
        }

        canvas.set_draw_color(Color::BLACK);
        canvas.clear();

        let left_boundary_i16 = left_boundary;
        canvas.box_(left_boundary_i16, 0, left_boundary_i16 - 2, STAGE_HEIGHT, Color::WHITE)?;

        let right_boundary_i16 = right_boundary;
        canvas.box_(right_boundary_i16, 0, right_boundary_i16 + 2, STAGE_HEIGHT, Color::WHITE)?;

        canvas.box_(player_pos.0, player_pos.1, player_pos.0 + player_size.0, player_pos.1 + player_size.1, Color::WHITE)?;

        for car in &traffic_cars {
            let lane_x = left_boundary + car.lane * LANE_WIDTH;
            canvas.box_(lane_x, car.y, lane_x + player_size.0, car.y + player_size.1, Color::WHITE)?;
        }

        canvas.present();
    }

    Ok(())
}