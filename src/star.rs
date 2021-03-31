
use nalgebra::{
    DimName, DefaultAllocator, base::allocator::Allocator,
};
use crate::{
    Float, Vector, Point,
    body::Body,
};

#[derive(new)]
pub struct Star<D>
    where
        D: DimName,
        DefaultAllocator: Allocator<Float, D>,
{
    p: Point<D>,
    v: Vector<D>,
    m: Float,
    pub radius: f32,    // More for drawing than anything really
}

impl<D> Star<D>
    where
        D: DimName,
        DefaultAllocator: Allocator<Float, D>,
{
}

impl<D> Body<D> for Star<D>
    where
        D: DimName,
        DefaultAllocator: Allocator<Float, D>,
{
    default_body_gets!(p, v, m);
}

