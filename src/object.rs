#[path="object2d.rs"]
mod object2d;
pub use object2d::Object2D;

#[path="object3d.rs"]
mod object3d;
pub use object3d::Object3D;

use drython::types::error::ErrorManager;
use drython::types::{Runner, VariableReference};
use crate::transform::Transform;

pub struct Object<'a>
{
    pub name: String,
    pub parent: Option<&'a Object<'a>>,
    pub children: Option<Vec<&'a mut dyn Transform>>,

    pub script_path: String,
    pub script: Option<Runner>,
    pub script_errors: ErrorManager
}

impl<'a> VariableReference for Object<'a>
{

}
