
use raylib::{prelude::Vector2, prelude::{Vector3, Quaternion}};

pub trait Transform
{
    
}

pub struct Transform2D
{
    pub pos: Vector2,
    pub rot: f32,
    pub scale: Vector2
}

impl Transform for Transform2D
{

}

pub struct Transform3D
{
    pub pos: Vector3,
    pub rot: Quaternion,
    pub scale: Vector3
}

impl Transform for Transform3D
{

}
