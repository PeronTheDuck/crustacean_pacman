use crate::controls::Gamepad;
use crate::dots::{Dot, DotMap};
use crate::entity::Direction;
use crate::entity::Entity;
use crate::map::Map;
use crate::render::{Render, Text};
use crate::sprite::Sprite;
use opengl_graphics::{GlGraphics, Texture as GlTexture};
use piston_window as pw;
use piston_window::Transformed;
use std::cell::RefCell;

pub struct App<'a> {
    pub board: GlTexture,
    pub entities: Vec<Entity<'a>>,
    pub player: usize,
    pub ghosts: [usize; 4],
    pub controls: Gamepad,
    pub debug: bool,
    //pub font: RefCell<Font>,
    pub score: [u32; 3],
    pub texts: Vec<Text<'a, String>>,
    pub dots: DotMap<'a>,
}

impl<'a> App<'a> {
    pub fn render(&mut self, args: pw::RenderArgs, mut gl: &mut GlGraphics) {
        let c = gl.draw_begin(args.viewport());
        self.draw(&mut gl, &c);
        gl.draw_end();
    }
    pub fn update(&mut self) {
        let dir = self.controls.get_one_direction();
        if dir != Direction::Stop {
            let player: &mut Entity = &mut self.entities[self.player];
            player.change_direction(dir);
        }
        for e in &mut self.entities {
            e.update_pos();
            let (node, distance) = e.map.get_nearest_node(e.pos);
            if distance < 3.0 {
                let old_node = e.node;
                e.change_node(node);

                if old_node != e.node {
                    println!(
                        "Updated node for {}. Now {}",
                        e.name.unwrap(),
                        e.node.unwrap()
                    );
                    println!(
                        "Changed node to {}. Valid directions now are {:#?}",
                        node, e.map.nodes[node].neighs
                    );
                }
            } else {
                e.node = None;
            }
            if e.name == Some("Pacman") {
                let nearest = self.dots.get_nearest_node(e.pos);
                if nearest.1 < 8.0 {
                    let node = self.dots.dots.remove(nearest.0);
                    self.score[0] += node.score as u32;
                }
            }
        }

        self.entities[self.player].sprite.animate();

        for i in 0..1 {
            if self.score[i * 2] > self.score[1] {
                self.score[1] = self.score[i * 2];
            }
        }

        for i in 0..3 {
            self.texts[i + 3].text = self.score[i].to_string();
        }
    }
    pub fn entities_update(&mut self, args: pw::ButtonArgs) {
        let player: &mut Entity = &mut self.entities[self.player];
        if args.state == pw::ButtonState::Press {
            if let pw::Button::Keyboard(key) = args.button {
                println!("Changing direction to {:#?}", key);
                let could = player.change_direction(match key {
                    pw::keyboard::Key::Up => Direction::Up,
                    pw::keyboard::Key::Right => Direction::Right,
                    pw::keyboard::Key::Down => Direction::Down,
                    pw::keyboard::Key::Left => Direction::Left,
                    _ => Direction::Stop,
                });
                if !could {
                    println!("Couldn't change direction");
                }
            }
        }
    }
}
impl<'a> Render for App<'a> {
    fn draw(&self, gl: &mut opengl_graphics::GlGraphics, c: &pw::Context) {
        let img = pw::Image::new();
        img.draw(&self.board, &pw::DrawState::default(), c.transform, gl);

        self.dots.draw(gl, &c);

        for e in &self.entities {
            e.draw(gl, &c);
        }

        for text in &self.texts {
            text.draw(gl, &c);
        }
    }
}
