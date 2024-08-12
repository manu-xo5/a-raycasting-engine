use crate::draw;
use crate::vector::Vector2;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
    id: &'static str,
    position: Vector2,
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
        let x = (point.x.clone() as i32) >> 6 << 6;
        let y = (point.y.clone() as i32) >> 6 << 6;

        if x == object.position.x as i32 && y == object.position.y as i32 {
            Some(object.position.to_owned())
        } else {
            None
        }
    }
}
