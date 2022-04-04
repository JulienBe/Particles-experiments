extern crate good_web_game as ggez;

use ggez::{Context, GameResult};
use ggez::cgmath::{Point2, Vector2};
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use ggez::timer;
use rand::prelude::ThreadRng;
use rand::{Rng, RngCore};

const WIDTH: usize = 240;
const HEIGHT: usize = 180;
const DIFFUSION_ENERGY: usize = 5;
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

struct MainState {
    spritebatch: graphics::spritebatch::SpriteBatch,
    energy: Vec<usize>,
    rnd: ThreadRng,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let image = graphics::Image::new(ctx, "square_white.png").unwrap();
        let batch = graphics::spritebatch::SpriteBatch::new(image);
        let s = MainState {
            spritebatch: batch,
            energy: vec![0; (WIDTH * HEIGHT) as usize],
            rnd: rand::thread_rng(),
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        for _i in 0..10 {
            let index = (self.rnd.next_u32() % self.energy.len() as u32) as usize;
            self.energy[index] += 1;
            if self.energy[index] >= DIFFUSION_ENERGY {
                self.energy[index] = 0;
            }
        }
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }
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
                    .color(COLORS[self.energy[y as usize * WIDTH + x as usize]])
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