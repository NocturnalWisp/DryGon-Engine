use crate::{object::TObject, generate_get_name, generate_get_id};

use raylib::math::Vector2;
use crate::transform::Transform2D;
use raylib::texture::Texture2D;
use crate::object::Object;

pub struct Object2D
{
    pub object: Object,

    pub sprite: Option<Texture2D>,
    pub transform: Transform2D,
}

impl TObject for Object2D
{
    fn new() -> Self
    {
        Object2D
        {
            object: Object::new(),
            sprite: None,
            transform: Transform2D
            {
                pos: Vector2::zero(),
                rot: 0.0,
                scale: Vector2::zero(),
            },
        }
    }

    generate_get_name!();
    generate_get_id!();

    fn get_obj(&mut self) -> &mut Object
    {
        &mut self.object
    }
}
