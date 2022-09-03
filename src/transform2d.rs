use crate::transform::Transform;
use raylib::prelude::Vector2;

pub struct Transform2D
{
    pub pos: Vector2,
    pub rot: f32,
    pub scale: Vector2
}

impl Transform for Transform2D
{

}
