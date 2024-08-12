use sdl2::{Sdl, video::Window };

pub struct Game {
    pub win: Window,
    pub sdl: Sdl,
} 

pub fn create_win() -> Result<Game, &'static str> {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let window = video
        .window("Windo", 800, 600)
        .build()
        .unwrap();

    Ok(Game {
        win: window,
        sdl: sdl,
    })
}
