use drython::types::{error::ErrorManager, VariableReference};
use raylib::math::Vector2;
use crate::transform::Transform2D;
use raylib::texture::Texture2D;
use crate::object::Object;

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
                script_errors: ErrorManager::new(),
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

impl<'a> VariableReference for Object2D<'a> {}
