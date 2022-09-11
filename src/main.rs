extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::{Button, ButtonEvent, ButtonState, Key};
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, EventLoop, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::{GlutinWindow as Window, GlutinWindow};
use opengl_graphics::{GlGraphics, OpenGL};

#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
}

impl Game {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 0.5, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            //Clears the screen
            clear(GREEN, gl);
        });

        self.snake.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.dir.clone();

        self.snake.dir = match btn {
            &Button::Keyboard(Key::W)
                if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::S)
                if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::A)
                if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::D)
                if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

pub struct Snake {
    pos_x: i32,
    pos_y: i32,
    dir: Direction,
}

impl Snake {
    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs) {
        use graphics::*;

        const RED: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        const SNAKE_SIZE: i32 = 20;

        let square = rectangle::square(
            (self.pos_x * SNAKE_SIZE) as f64,
            (self.pos_y * SNAKE_SIZE) as f64,
            SNAKE_SIZE as f64);

        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            rectangle(RED, square, transform, gl);
        });
    }

    fn update(&mut self) {
        match self.dir {
            Direction::Right => self.pos_x += 1,
            Direction::Left => self.pos_x -= 1,
            Direction::Down => self.pos_y += 1,
            Direction::Up => self.pos_y -= 1,
        }
    }
}

fn main() {
    //Selecting the OpenGL version
    let opengl = OpenGL::V3_2;

    //Creting a Glutin window
    let mut window: GlutinWindow = WindowSettings::new("SnakeR", [600, 400])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        //GRID 30x20
        //snake MAX -> pos_x: 29, pos_y: 19
        snake: Snake { pos_x: 0, pos_y: 0, dir: Direction::Right },
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            game.render(&args);
        }

        if let Some(_args) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}
