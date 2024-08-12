use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::io::Error;
use std::time::Duration;

mod vector;
use vector::Vector2;
mod draw;
mod win;
use win::create_win;
mod player;
use crate::player::Player;
mod level;
use level::{builder::Builder, Level, Object};

mod raycast;

fn main() -> Result<(), String> {
    let mut items: [Object; 40] = [Object::none(); 40];
    for i in 0..40 {
        items[i] = Object::new("wall-1", (i as i32 * 20, 0));
    }

    let game = create_win()?;
    let mut canvas = game.win.into_canvas().build().unwrap();
    let mut event_pipe = game.sdl.event_pump()?;
    let mut player = Player::new(Vector2::new(0.0, 0.0));

    let mut builder = Builder::new().unwrap();
    builder.items = Builder::load();
    let mut mouse_position = Vector2::new(0.0, 0.0);

    'running: loop {
        canvas.clear();

        let mouse_state = event_pipe.mouse_state();
        for event in event_pipe.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::ESCAPE),
                    ..
                }
                | Event::Quit { .. } => {
                    println!("quitting");
                    break 'running;
                }

                Event::MouseMotion { x, y, .. } => {
                    mouse_position.x = x as f64;
                    mouse_position.y = y as f64;
                }

                _ => {}
            };
            Builder::handle_event(&event, mouse_state, &mut builder.items);
            player.handle_event(&event);
        }

        canvas.set_draw_color((0, 0, 0));
        // draw bg
        draw::rect(&Vector2::new(0.0, 0.0), 800, 600, &mut canvas);
        // draw floor
        canvas.set_draw_color((100, 100, 100));
        draw::filled_rect(&Vector2::new(0.0, 300.0), 800, 600, &mut canvas);

        raycast::draw_rays(
            &player.position,
            &Vector2::from_angle(player.angle).add(&player.position),
            &mut canvas,
            &(&builder.items)
                .iter()
                .map(|x| Object::new("wall-1", (x.1, x.2)))
                .collect::<Vec<Object>>(),
        );

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    Ok(())
}
