use sdl2::render::BlendMode;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::draw;
use crate::level::{Object, CELL, SCREEN_H, SCREEN_W};
use crate::player::Player;
use crate::Vector2;

const EPS: f64 = 1e-6;
const FOV: f64 = 60.0;
const FAR_CLIPPING: f64 = 400.0;

pub fn is_collision(p1: &Vector2, p2: &Vector2, size: f64, j: &mut u32) -> bool {
    *j += 1;
    if *j > 500 {
        // println!("{} {}: {} {}", p1.x, p1.y, p2.x, p2.y);
        *j = 0;
    }
    if p1.x > p2.x && p1.x < p2.x + size && p1.y > p2.y && p1.y < p2.y + size {
        true
    } else {
        false
    }
}

pub fn draw_rays(p1_: &Vector2, p2_: &Vector2, canvas: &mut Canvas<Window>, level: &Vec<Object>) {
    let mut i = 0.0;

    while i < FOV {
        //let angle = p1.angle.clone() + 4.0; //+ i - (FOV / 2.0);

        let d = p2_.sub(&p1_);
        let angle = i - (FOV / 2.0);
        let ax = d.x * angle.to_radians().cos() - d.y * angle.to_radians().sin();
        let ay = d.x * angle.to_radians().sin() + d.y * angle.to_radians().cos();

        let mut p2 = Vector2::new(ax, ay).scale(CELL * 2.0).add(&p2_);

        let mut p1 = p1_.clone();

        //draw::line(&p1_, &p2, canvas);

        'collision: for _i in 0..(SCREEN_W * 2.0) as u8 {
            for object in level {
                // 260,220
                // 256 220 | 260 223 | 276 240
                if p2.x > 800.0 || p2.x < 0.0 {
                    break 'collision;
                }
                if p2.y > 600.0 || p2.y < 0.0 {
                    break 'collision;
                }
                if p2.sub(&p1_).length() > FAR_CLIPPING {
                    break 'collision;
                }

                if Object::is_at(&p2, &object).is_some() {
                    let w = SCREEN_W / FOV;
                    let v = p2.sub(&p1_);
                    let d = p2_.sub(&p1_).norm();
                    let h = (SCREEN_H / v.dot(&d)) * 40.0;
                    let a = ((h / FAR_CLIPPING) * 255.0) as u8;

                    // canvas.set_draw_color((200, 0, 0));
                    // draw::rect(&p2.sub(&Vector2::new(5.0, 5.0)), 10, 10, canvas);
                    // draw::line(&p2, &p1_.sub(&p2_).norm().scale(l).add(&p2), canvas);

                    if p2.x % CELL == 0.0 {
                        canvas.set_draw_color((a, 0, 0));
                    } else {
                        canvas.set_draw_color((0, a, 0));
                    };

                    draw::filled_rect(
                        &Vector2::new(i * w, (SCREEN_H - h) * 0.5),
                        w as u32,
                        h as u32,
                        canvas,
                    );

                    canvas.set_draw_color((0, 0, 0));
                    break 'collision;
                }
            }

            let p3 = ray_step(&p1, &p2);
            p1 = p2;
            p2 = p3;
        }
        i += 0.5;
    }
}

fn snap(x: &f64, dx: &f64) -> f64 {
    let eps = EPS * (dx.signum());

    if *dx > 0.0 {
        (((*x).clone() + eps) / CELL).ceil() * CELL
    } else if *dx < 0.0 {
        (((*x).clone() + eps) / CELL).floor() * CELL
    } else {
        *x
    }
}

pub fn ray_step(p1: &Vector2, p2: &Vector2) -> Vector2 {
    let d = p2.sub(&p1);

    let mut p3: Vector2;

    if d.x != 0.0 {
        let k = d.y / d.x;
        let c = p1.y - k * p1.x;

        let px = snap(&p2.x, &d.x);
        let py = px * k + c;
        p3 = Vector2::new(px, py);

        if k != 0.0 {
            let py = snap(&p2.y, &d.y);
            let px = (py - c) / k;
            let p3t = Vector2::new(px, py);

            if p2.distance_to(&p3t) < p2.distance_to(&p3) {
                p3 = p3t;
            }
        }
    } else {
        let py = snap(&p2.y, &d.y);
        let px = p2.x;
        p3 = Vector2::new(px, py);
    }

    p3
}
