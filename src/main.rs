#[macro_use]
extern crate derive_new;

#[macro_use]
pub mod body;
pub mod star;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

use nalgebra::{
    DimName, DefaultAllocator, base::allocator::Allocator,
};
use std::rc::Rc;
use std::cell::{RefCell, RefMut};


use body::Body;
use star::Star;


pub type Float = f64;   // Precision of floats
pub type Point<D> = nalgebra::Point<Float, D>;
pub type Vector<D> = nalgebra::VectorN<Float, D>;



const GRAV_CONST: Float = 0.1; // 6.67408e-11;
const DT: Float = 1.0/60.0;



pub struct App<D>
    where
        D: DimName,
        DefaultAllocator: Allocator<Float, D>,
{
    gl: GlGraphics,
    stars: Rc<RefCell<Vec<Star<D>>>>,
}

impl<D> App<D>
    where
        D: DimName,
        DefaultAllocator: Allocator<Float, D>,
{
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        // Clear screen
        self.gl.draw(args.viewport(), |c, gl| { clear([0.0; 4], gl) });

        {
            let stars = self.stars.borrow();
            
            for i in 0..stars.len() {
                let (x, y) = {
                    let p = stars[i].position();
                    (p[0] as f64, p[1] as f64)
                };

                self.gl.draw(args.viewport(), |c, gl| {
                    let transform = c
                        .transform
                        .trans(x, y);

                    ellipse([1.0, 1.0, 1.0, 1.0], [stars[i].radius as f64 * 2.0; 4], transform, gl);
                });
            }
        }

    }

    fn update(&mut self, args: &UpdateArgs) {
        Self::apply_gravity(&mut self.stars.borrow_mut());
        Self::update_bodies(&mut self.stars.borrow_mut())
    }

    fn apply_gravity<B: Body<D>>(bodies: &mut Vec<B>) {
        for i in 0..bodies.len()-1 {
            for j in i+1..bodies.len() {
                let force = bodies[i].newtonian_force(&bodies[j]);
                bodies[i].apply_force(&force, DT);
                bodies[j].apply_force(&(-force), DT);
            }
        }
    }

    fn update_bodies<B: Body<D>>(bodies: &mut Vec<B>) {
        for b in bodies.iter_mut() {
            b.update_position(DT);
        }
    }

}


pub fn main() {
    use nalgebra::{Point2, Vector2};

    let mut stars = Rc::new(RefCell::new(vec![
        Star::new(
            Point2::new(100.0, 100.0),
            Vector2::new(10.0, 0.0),
            5000.0,
            10.0,
        ),
        Star::new(
            Point2::new(100.0, 150.0),
            Vector2::new(10.0, 0.0),
            5000.0,
            10.0,
        ),
    ]));

    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new("Orbits", [800, 600])
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        stars,
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            app.render(&args);
        }

        if let Some(args) = e.update_args() {
            app.update(&args);
        }
    }
}