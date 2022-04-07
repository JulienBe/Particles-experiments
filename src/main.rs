extern crate good_web_game as ggez;

use std::cmp::{max, min};
use ggez::{Context, GameResult};
use ggez::cgmath::{Point2, Vector2};
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::miniquad::KeyCode::End;
use ggez::timer;
use rand::{Rng, RngCore};
use rand::prelude::ThreadRng;

const WIDTH: usize = 132;
const HEIGHT: usize = 99;
const PIXEL_SIZE: f32 = 4.0;
const PIXEL_GAP: f32 = 1.0;
const PIXEL_POS: f32 = PIXEL_SIZE + PIXEL_GAP;
const PALETTE_5: Color = Color { r: 255. / 255., g: 204. / 255., b: 170. / 255., a: 1. };
const PALETTE_4: Color = Color { r: 191. / 255., g: 112. / 255., b: 034. / 255., a: 1. };
const PALETTE_3: Color = Color { r: 126. / 255., g: 037. / 255., b: 083. / 255., a: 1. };
const PALETTE_2: Color = Color { r: 029. / 255., g: 043. / 255., b: 083. / 255., a: 1. };
const PALETTE_1: Color = Color { r: 000. / 255., g: 000. / 255., b: 000. / 255., a: 1. };
const COLORS: [Color; 5] = [
    PALETTE_1,
    PALETTE_2,
    PALETTE_3,
    PALETTE_4,
    PALETTE_5,
];
const MAX_COLOR_INDEX: usize = COLORS.len() - 1;
const DIFFUSION_ENERGY: u8 = MAX_COLOR_INDEX as u8;

#[derive(Clone)]
struct Particle {
    energy: u8,
    color: usize,
    index: usize,
}

impl Particle {
    fn new() -> Particle {
        Particle { energy: std::cmp::max(2, (rand::thread_rng().next_u32() % MAX_COLOR_INDEX as u32) as u8), color: 0, index: 0 }
    }
}

struct GridState {
    width: usize,
    height: usize,
    particles: Vec<Particle>,
}

impl GridState {
    fn new(width: usize, height: usize) -> GridState {
        let mut particles = vec![Particle::new(); (width * height) as usize];
        particles.iter_mut().enumerate().for_each(|(i, p)| {
            p.index = i
        });
        GridState {
            width,
            height,
            particles,
        }
    }
    fn left_particle(&self, origin: usize) -> usize {
        if origin == 0 {
            return self.width - 1;
        }
        let mut left_index = origin - 1;
        if left_index % self.width == self.width - 1 {
            left_index += self.width;
        }
        left_index
    }
    fn right_particle(&self, origin: usize) -> usize {
        if origin == self.particles.len() - 1 {
            return self.particles.len() - self.width;
        }
        let mut right_index = origin + 1;
        if right_index % self.width == 0 {
            right_index -= self.width;
        }
        right_index
    }
    fn up_particle(&self, origin: usize) -> usize {
        if origin < self.width {
            return self.width * (self.height - 1) + origin;
        }
        origin - self.width
    }
    fn down_particle(&self, origin: usize) -> usize {
        if origin >= self.width * (self.height - 1) {
            return origin % self.width;
        }
        origin + self.width
    }
}

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
    grid: GridState,
    rnd: ThreadRng,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "square_white.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);

        let s = MainState {
            spritebatch: batch,
            grid: GridState::new(WIDTH, HEIGHT),
            rnd: rand::thread_rng(),
        };
        Ok(s)
    }

    fn update_particles(&mut self) {
        let mut over_energy_limit_indexes: Vec<usize> = vec![];
        self.grid.particles.iter_mut()
            .filter(|p| p.energy > DIFFUSION_ENERGY)
            .for_each(|p| over_energy_limit_indexes.push(p.index)); // fighting the borrow checker a bit here. Future me, there is probably a better way to do this

        over_energy_limit_indexes.iter().for_each(|i| {
            self.grid.particles[*i].energy = 0;
            self.grid.particles[*i].color = MAX_COLOR_INDEX;

            let neighbors_indexes = [
                self.grid.left_particle(*i),
                self.grid.right_particle(*i),
                self.grid.up_particle(*i),
                self.grid.down_particle(*i),
            ];
            neighbors_indexes.iter().for_each(|neighbors_index| {
                self.grid.particles[*neighbors_index].energy += 1;
                self.grid.particles[*neighbors_index].color = min(self.grid.particles[*neighbors_index].energy as usize, MAX_COLOR_INDEX);
            })
        });
    }

    fn add_energy(&mut self) {
        // Only add energy if there are no reactions going on so we can peacefully observe the current reactions
        if self.grid.particles.iter().all(|p| p.color == 0) {
            let index = (self.rnd.next_u32() % self.grid.particles.len() as u32) as usize;
            self.grid.particles[index].energy += 1;
        }
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 20 == 0 {
            self.update_particles();
        }

        if timer::ticks(ctx) % 30 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
            self.grid.particles.iter_mut()
                .filter(|p| p.color > 0)
                .for_each(|p| p.color -= 1);
        }

        self.add_energy();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::Color::BLACK);

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let x = x as f32;
                let y = y as f32;
                let p = graphics::DrawParam::new()
                    .dest(Point2::new(x * PIXEL_POS, y * PIXEL_POS))
                    .color(COLORS[self.grid.particles[y as usize * WIDTH + x as usize].color])
                    .scale(Vector2::new(PIXEL_SIZE, PIXEL_SIZE));
                self.spritebatch.add(p);
            }
        }
        graphics::draw(ctx, &self.spritebatch, graphics::DrawParam::new())?;
        self.spritebatch.clear();

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    ggez::start(
        ggez::conf::Conf::default()
            .cache(Some(include_bytes!("resource.tar"))),
        |mut context| Box::new(MainState::new(&mut context).unwrap()),
    )
}

#[cfg(test)]
mod tests {
    use crate::GridState;

    const W: usize = 4;
    const H: usize = 3;
    const TOP_LEFT: usize = 0;
    const TOP_RIGHT: usize = W - 1;
    const BOTTOM_LEFT: usize = (W * H) - W;
    const BOTTOM_RIGHT: usize = (W * H) - 1;

    #[test]
    fn left_particle() {
        let grid = GridState::new(W, H);
        assert_eq!(grid.left_particle(TOP_LEFT), TOP_RIGHT);
        assert_eq!(grid.left_particle(TOP_RIGHT), TOP_RIGHT - 1);
        assert_eq!(grid.left_particle(BOTTOM_LEFT), BOTTOM_RIGHT);
        assert_eq!(grid.left_particle(BOTTOM_RIGHT), BOTTOM_RIGHT - 1);
    }

    #[test]
    fn right_particle() {
        let grid = GridState::new(W, H);
        assert_eq!(grid.right_particle(TOP_LEFT), TOP_LEFT + 1);
        assert_eq!(grid.right_particle(TOP_RIGHT), TOP_LEFT);
        assert_eq!(grid.right_particle(BOTTOM_LEFT), BOTTOM_LEFT + 1);
        assert_eq!(grid.right_particle(BOTTOM_RIGHT), BOTTOM_LEFT);
    }

    #[test]
    fn up_particle() {
        let grid = GridState::new(W, H);
        assert_eq!(grid.up_particle(TOP_LEFT), BOTTOM_LEFT);
        assert_eq!(grid.up_particle(TOP_RIGHT), BOTTOM_RIGHT);
        assert_eq!(grid.up_particle(BOTTOM_LEFT), BOTTOM_LEFT - W);
        assert_eq!(grid.up_particle(BOTTOM_RIGHT), BOTTOM_RIGHT - W);
    }

    #[test]
    fn down_particle() {
        let grid = GridState::new(W, H);
        assert_eq!(grid.down_particle(TOP_LEFT), TOP_LEFT + W);
        assert_eq!(grid.down_particle(TOP_RIGHT), TOP_RIGHT + W);
        assert_eq!(grid.down_particle(BOTTOM_LEFT), TOP_LEFT);
        assert_eq!(grid.down_particle(BOTTOM_RIGHT), TOP_RIGHT);
    }
}