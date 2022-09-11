extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::{GlutinWindow as Window, GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL};

pub struct Game {
    gl: GlGraphics,
}

impl Game {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 0.0];

        self.gl.draw(args.viewport(), |c, gl| {
            //Clears the screen
            clear(GREEN, gl);
        })
    }
}

fn main() {
    //Selecting the OpenGL version
    let opengl = OpenGL::V3_2;

    //Creting a Glutin window
    let mut window: GlutinWindow = WindowSettings::new("SnakeR", [750, 500])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl)
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        /*if let Some(args) = e.update_args() {
            app.update(&args);
        }*/
    }
}
