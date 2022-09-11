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
    pub gl: GlGraphics,
    pub snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs){
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c,gl| {
            //Clears the screen
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }
}

pub struct Snake {
    pos_x: i32,
    pos_y: i32,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let square = rectangle::square(self.pos_x as f64, self.pos_y as f64, 20.0);

        gl.draw(args.viewport(), |c,gl| {
            let transform = c.transform;

            rectangle(RED, square, transform, gl);
        });
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
        gl: GlGraphics::new(opengl),
        snake: Snake{pos_x: 50, pos_y: 50},
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
