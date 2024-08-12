use crate::draw;
use crate::vector::Vector2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub const CELL: f64 = 20.0;
pub const SCREEN_W: f64 = 800.0;
pub const SCREEN_H: f64 = 600.0;

pub mod builder;

pub struct Level {
    pub items: [Object; 40],
}

impl Level {
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        for o in self.items.as_slice() {
            canvas.set_draw_color(Color::RGB(255, 255, 255));
            draw::rect(&o.position, 20, 20, canvas);
        }
    }
}

#[derive(Clone, Copy)]
pub struct Object {
    pub id: &'static str,
    pub position: Vector2,
}

impl Object {
    pub fn none() -> Object {
        Object {
            id: "wall-1",
            position: Vector2::new(0.0, 0.0),
        }
    }

    pub fn new(id: &'static str, coords: (i32, i32)) -> Object {
        Object {
            id,
            position: Vector2::new(coords.0 as f64, coords.1 as f64),
        }
    }

    pub fn is_at(point: &Vector2, object: &Object) -> Option<Vector2> {
        let x = (point.x.clone() / CELL) * CELL;
        let y = (point.y.clone() / CELL) * CELL;

        if x >= object.position.x
            && x <= object.position.x + CELL
            && y >= object.position.y
            && y <= object.position.y + CELL
        {
            Some(object.position.to_owned())
        } else {
            None
        }
    }
}
