use crate::transform::Transform;
use raylib::math::Quaternion;
use raylib::math::Vector3;

pub struct Transform3D
{
    pub pos: Vector3,
    pub rot: Quaternion,
    pub scale: Vector3
}

impl Transform for Transform3D
{

}
