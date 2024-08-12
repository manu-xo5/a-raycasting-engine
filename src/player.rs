use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::draw;
use crate::vector::Vector2;
pub struct Player {
    pub position: Vector2,
    pub accel: u8,
    pub angle: f64,
}

impl Player {
    pub fn new(position: Vector2) -> Player {
        Player {
            position,
            accel: 4,
            angle: 0.0,
        }
    }

    pub fn handle_event(&mut self, event: &Event) {
        match *event {
            Event::KeyDown {
                keycode: Some(Keycode::A),
                ..
            } => self.strafe_left(),

            Event::KeyDown {
                keycode: Some(Keycode::W),
                ..
            } => self.move_forward(),

            Event::KeyDown {
                keycode: Some(Keycode::S),
                ..
            } => self.move_backward(),

            Event::KeyDown {
                keycode: Some(Keycode::D),
                ..
            } => self.strafe_right(),

            Event::KeyDown {
                keycode: Some(Keycode::L),
                ..
            } => self.rotate(-1),

            Event::KeyDown {
                keycode: Some(Keycode::J),
                ..
            } => self.rotate(1),

            Event::KeyDown {
                keycode, repeat, ..
            } => {
                println!("{:?} r: {}", keycode, repeat);
            }

            _ => {
                if self.accel > 0 {
                    self.accel -= 1;
                }
            }
        }
    }

    pub fn render(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color((25, 25, 255));
        draw::rect(
            &Vector2::new(&self.position.x - 10.0, &self.position.y - 10.0),
            20,
            20,
            canvas,
        );

        let l = Vector2::from_angle(self.angle - 90.0).scale(50.0).add(&self.position);
        let r = Vector2::from_angle(self.angle + 90.0).scale(50.0).add(&self.position);
        canvas.set_draw_color((25, 25, 255));
        draw::line(&l, &r, canvas);
    }

    fn start_accel(&mut self) {
        if self.accel < 5 {
            self.accel += 2;
        }
    }

    fn rotate(&mut self, dir: i32) {
        let rt = 2.0;
        self.angle += (dir as f64) * rt;
    }

    fn strafe_left(&mut self) {
        self.start_accel();

        let new_p = self
            .position
            .sub(&Vector2::from_angle(self.angle - 90.0).scale(self.accel as f64));

        self.position.x = new_p.x;
        self.position.y = new_p.y;
    }

    fn strafe_right(&mut self) {
        self.start_accel();

        let new_p = self
            .position
            .sub(&Vector2::from_angle(self.angle + 90.0).scale(self.accel as f64));
        self.position.x = new_p.x;
        self.position.y = new_p.y;
    }

    fn move_backward(&mut self) {
        self.start_accel();

        let new_p = self
            .position
            .sub(&Vector2::from_angle(self.angle).scale(self.accel as f64));
        self.position.x = new_p.x;
        self.position.y = new_p.y;
    }

    fn move_forward(&mut self) {
        self.start_accel();

        let a = self.angle;
        let x = a.to_radians().cos() * (self.accel as f64);
        let y = a.to_radians().sin() * (self.accel as f64);
        self.position.x += x;
        self.position.y -= y;
    }
}
