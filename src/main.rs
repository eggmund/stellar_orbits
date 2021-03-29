#[macro_use]
extern crate derive_new;

#[macro_use]
pub mod body;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

use nalgebra::{
    DimName, Point, VectorN, DefaultAllocator, base::allocator::Allocator, DimNameMul,
};

use body::Body;

pub type Float = f64;   // Precision of floats

const GRAV_CONST: Float = 6.67408e-11;


pub trait Dimension<D: DimName>: DimName + DimNameMul<D> {}


#[derive(new)]
pub struct Star<D>
    where
        D: Dimension<D>,
        DefaultAllocator: Allocator<Float, D>,
{
    p: Point<Float, D>,
    v: VectorN<Float, D>,
    m: Float,
}

impl<D> Star<D>
    where
        D: Dimension<D>,
        DefaultAllocator: Allocator<Float, D>,
{
}

impl<D> Body<D> for Star<D>
    where
        D: Dimension<D>,
        DefaultAllocator: Allocator<Float, D>,
{
    default_body_gets!(p, v, m);
}


fn apply_gravity<D: Dimension<D>>(bodies: Vec<Box<dyn Body<D>>>) {

}

fn update_bodies<D: Dimension<D>>(bodies: Vec<Box<dyn Body<D>>>) {

}


pub fn main() {
    


    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...



        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}