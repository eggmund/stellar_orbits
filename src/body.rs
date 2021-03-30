use nalgebra::{DefaultAllocator, base::allocator::Allocator, DimName};

use crate::{Float, Vector, Point, GRAV_CONST};

pub trait Body<D>
where
    D: DimName,
    DefaultAllocator: Allocator<Float, D>,
{
    fn position(&self) -> &Point<D>;
    fn position_mut(&mut self) -> &mut Point<D>;
    fn velocity(&self) -> &Vector<D>;
    fn velocity_mut(&mut self) -> &mut Vector<D>;
    fn mass(&self) -> &Float;
    fn mass_mut(&mut self) -> &mut Float;

    fn newtonian_force(&self, other: &dyn Body<D>) -> Vector<D> {
        // F = GmM/r^2
        // F_vec = (GmM/r^2) * r_norm
        // F_vec = (GmM/r^3) * r_vec
        let r = other.position() - self.position();
        let distance = r.norm();

        r * GRAV_CONST * ((*self.mass() * *other.mass()) / (distance * distance * distance))
    }

    fn apply_force(&mut self, force: &Vector<D>, dt: Float) {
        // F = dp/dt
        // dp = F dt
        // dp = m dv = F dt
        // dv = F dt/m
        let dv: Vector<D> = force * (dt / self.mass());
        *self.velocity_mut() += dv;
    }

    fn update_position(&mut self, dt: Float) {
        // v = dx/dt
        // v dt = dx
        let dr = self.velocity() * dt;
        *self.position_mut() += dr;
    }
}

// Macro for implementing simple functions for Body trait
macro_rules! default_body_gets {
    ($position:ident, $velocity:ident, $mass:ident) => {
        fn position(&self) -> &Point<D> { &self.$position }
        fn position_mut(&mut self) -> &mut Point<D> { &mut self.$position }
        fn velocity(&self) -> &Vector<D> { &self.$velocity }
        fn velocity_mut(&mut self) -> &mut Vector<D> { &mut self.$velocity }
        fn mass(&self) -> &Float { &self.$mass }
        fn mass_mut(&mut self) -> &mut Float { &mut self.$mass }
    };
}
