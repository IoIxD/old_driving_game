extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::{time::{Duration,SystemTime}};

use noise::{Perlin, NoiseFn, Seedable};

const WIDTH: i32 = 640;
const HEIGHT: i32 = 480;
const SCALE: i32 = 1;

const SPEED: f64 = 0.1;


const SAMPLES: usize = 50;

type OurNoiseFn = Perlin;

struct GameObject {
    pub x: f64,
    pub y: f64,
}
impl GameObject {
    fn move_left(&mut self) {self.x = self.x - SPEED; }
    fn move_up(&mut self) {self.y = self.y - SPEED; }
    fn move_right(&mut self) {self.x = self.x + SPEED; }
    fn move_down(&mut self) {self.y = self.y + SPEED; }
}

#[derive(Clone)]
#[derive(Copy)]
struct OurNoise {
    noise: [OurNoiseFn; SAMPLES]
}

impl OurNoise {
    fn new() -> OurNoise {
        let now = SystemTime::now();
        
        let mut vals: Vec<OurNoiseFn> = vec!();
        for _ in 0..SAMPLES {
            let seed = match now.elapsed() {
                Ok(elapsed) => (elapsed.as_nanos() / 4) as u32,
                Err(err) => panic!("{}",err),
            };
            let n = OurNoiseFn::new();
            n.set_seed(seed);
            vals.push(n);
        }
        let vals_array: [OurNoiseFn; SAMPLES] = vals
                                          .into_iter()
                                          .collect::<Vec<OurNoiseFn>>()
                                          .try_into()
                                          .unwrap();
        OurNoise {
            noise: vals_array
        }
    }
    fn at(&mut self, x: f64, y: f64) -> f64 {
        let mut final_value = 0.0;
        for i in 0..SAMPLES {
            final_value += self.noise[i].get([x,y])
        }
        final_value / (SAMPLES as f64)
    }
}

fn main() {
    let mut cam = GameObject{x:0.1, y:0.1};

    let mut road = OurNoise::new();

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("Hi", WIDTH as u32, HEIGHT as u32)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();
        canvas.set_draw_color(Color::RGB(52, 168, 235));
        
        draw_road(&mut canvas,(cam.x, cam.y),&mut road);
        canvas.present();
        
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'running
                    },
                Event::KeyDown { keycode: Some(Keycode::Left), .. } => {
                    cam.move_left();
                    },
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    cam.move_up();
                    },
                Event::KeyDown { keycode: Some(Keycode::Down), .. } => {
                    cam.move_down();
                    },
                Event::KeyDown { keycode: Some(Keycode::Right), .. } => {
                    cam.move_right();
                    },
                    
                _ => {}
            }
        }

        ::std::thread::sleep(Duration::new(0, (2 ^ 31) as u32 / 60));
    }
}

fn draw_road(canvas: &mut Canvas<Window>, cam_pos: (f64, f64), ournoise: &mut OurNoise) {
    canvas.clear();

    let mut x_pos: f32 = 0.0;
    let mut y_pos: f32 = (HEIGHT-(HEIGHT/3)) as f32;    
    let mut last_y: i32 = 0;
    let mut h_mul: f32 = 1.0;
    let mut v_mul: f32 = 0.0;

    let mut y = y_pos as i32;
    while y_pos <= (HEIGHT) as f32 {
        let mut x = 0;
        // as we fill in the space vertically between our new spot and our lost one...
        while y > last_y {  
            while x < WIDTH {
               
                let val = 
                    noise_val_to_u8(
                        ournoise.at(
                            x as f64+cam_pos.0 + 0.1, 
                            y as f64+cam_pos.1 + 0.1
                        )
                    );

                canvas.set_draw_color(
                    Color::RGB(val, val, val)
                );
                
                for ys in y..y+(SCALE+(v_mul as i32)) {
                    for xs in x..x+(SCALE+(h_mul as i32)) {
                        canvas.draw_point((xs, ys)).
                        expect("Failed to draw");
                    }
                }
                x += SCALE+(h_mul as i32);
            }
            y -= SCALE+(v_mul as i32);
        }

        last_y = y_pos as i32;

        x_pos += (SCALE as f32)*2.0;
        y_pos += (SCALE as f32)+v_mul;
        
        h_mul = x_pos.sqrt().ceil();
        v_mul = ((y_pos * SCALE as f32).sqrt() - 15.0).ceil();

        y = y_pos as i32;
    };
}

#[inline]
fn noise_val_to_u8(value: f64) -> u8 {
    ((value % 1.0) * 255.0).ceil() as u8
}