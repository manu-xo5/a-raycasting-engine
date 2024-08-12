use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::draw;
use crate::player::Player;
use crate::Vector2;

const EPS: f64 = 1e-3;
const CELL: f64 = 20.0;

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

pub fn draw_rays(player: &Player, canvas: &mut Canvas<Window>) {
    let fov = 60.0;
    let mut i = 0.0;

    // let cell_size = Vector2::new(20.0, 20.0);

    while i < 1.0 {
        i += 1.0;
        let angle = player.angle.clone(); // + i - (fov / 2.0);

        let angle = Vector2::new(angle.to_radians().cos(), angle.to_radians().sin() * -1.0)
            .add(&player.position);

        canvas.set_draw_color((0, 0, 200));
        draw::line(
            &player.position,
            &angle
                .sub(&player.position)
                .norm()
                .scale(CELL * 8.0)
                .add(&angle),
            canvas,
        );

        let p3 = ray_step(
            player,
            &angle.sub(&player.position).norm().scale(40.0).add(&angle),
        );
        canvas.set_draw_color((200, 0, 0));
        draw::rect(&p3.sub(&Vector2::new(5.0, 5.0)), 10, 10, canvas);

        canvas.set_draw_color((0, 0, 0));
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

pub fn ray_step(player: &Player, p2: &Vector2) -> Vector2 {
    let d = p2.sub(&player.position);

    let mut p3: Vector2;

    if d.x != 0.0 {
        let k = d.y / d.x;
        let c = player.position.y - k * player.position.x;

        let px = snap(&p2.x, &d.x);
        let py = px * k + c;
        p3 = Vector2::new(px, py);

        if k != 0.0 {
            let py = snap(&p2.y, &d.y);
            let px = (py - c) / k;
            let p3t = Vector2::new(px, py);
            if p2.distanceTo(&p3t) < p2.distanceTo(&p3) {
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
