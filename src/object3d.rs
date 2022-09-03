use crate::transform::Transform3D;
use crate::object::Object;

pub struct Object3D<'a>
{
    object: Object<'a>,

    transform: Transform3D,
}
