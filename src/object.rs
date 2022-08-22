use drython::types::Parser;
use raylib::{prelude::Vector2, texture::Texture2D};

use crate::transform::Transform;

use super::transform::{Transform2D, Transform3D};

pub struct Object<'a>
{
    pub name: String,
    pub parent: Option<&'a Object<'a>>,
    pub children: Option<Vec<&'a mut dyn Transform>>,

    pub script_path: String,
    pub script: Option<Parser>
}

pub struct Object2D<'a>
{
    pub object: Object<'a>,

    pub sprite: Option<Texture2D>,
    pub transform: Transform2D,
}

impl<'a> Object2D<'a>
{
    pub fn new() -> Object2D<'a>
    {
        Object2D
        {
            object: Object
            {
                name: String::new(),
                parent: None,
                children: None,

                script_path: String::new(),
                script: None,
            },
            sprite: None,
            transform: Transform2D
            {
                pos: Vector2::zero(),
                rot: 0.0,
                scale: Vector2::zero(),
            },
        }
    }
}


pub struct Object3D<'a>
{
    object: Object<'a>,

    transform: Transform3D,
}
