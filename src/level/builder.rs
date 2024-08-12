use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::{MouseButton, MouseState};
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::draw;
use crate::vector::Vector2;
use std::fs::File;
use std::io::prelude::*;

type Coord = (String, i32, i32);

pub struct Builder {
    pub items: Vec<Coord>,
}

impl Builder {
    pub fn new() -> Result<Builder, std::io::Error> {
        Ok(Builder { items: vec![] })
    }

    pub fn load() -> Vec<Coord> {
        let mut file = File::open("foo.txt").unwrap();
        let mut data = String::new();

        file.read_to_string(&mut data).unwrap();

        return data
            .lines()
            .map(|s| {
                let mut tup = s.split(',').into_iter();
                let id = String::from(tup.next().unwrap_or(""));
                let x = tup.next().unwrap_or("0").parse::<i32>().unwrap_or(0);
                let y = tup.next().unwrap_or("0").parse::<i32>().unwrap_or(0);

                (id, x, y)
            })
            .collect();
    }

    pub fn handle_event(event: &Event, state: MouseState, items: &mut Vec<Coord>) {
        match event {
            Event::MouseButtonDown {
                mouse_btn: MouseButton::Left,
                ..
            } => {
                //let file = File::open("foo.txt").unwrap();
                let x = (state.x() / 20) * 20;
                let y = (state.y() / 20) * 20;
                items.push(("wall-1".to_string(), x, y));
            }

            Event::MouseButtonDown {
                mouse_btn: MouseButton::Right,
                ..
            } => {
                let x = (state.x() / 20) * 20;
                let y = (state.y() / 20) * 20;
                for i in 0..items.len() {
                    if items[i].1 == x && items[i].2 == y {
                        items.remove(i);
                        break;
                    }
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::O),
                ..
            } => {
                let mut file = File::create("foo.txt").unwrap();

                let coords = items
                    .into_iter()
                    .map(|x| {
                        String::from(
                            x.0.to_owned()
                                + ","
                                + x.1.to_string().as_str()
                                + ","
                                + x.2.to_string().as_str()
                                + "\n",
                        )
                    })
                    .collect::<String>();

                println!("{}", coords);
                file.write_all(coords.as_bytes()).unwrap();
            }
            _ => {}
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for i in &self.items {
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            draw::rect(&Vector2::new(i.1 as f64, i.2 as f64), 20, 20, canvas);
        }
    }
}
