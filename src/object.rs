use downcast_rs::Downcast;

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

#[path="object2d.rs"]
mod object2d;
pub use object2d::Object2D;

#[path="object3d.rs"]
mod object3d;
pub use object3d::Object3D;

use crate::drython_extensions::DrythonExRef;

static OBJECT_COUNTER: AtomicUsize = AtomicUsize::new(1);
pub fn generate_object_id() -> usize
{
    OBJECT_COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub trait TObject: Downcast + DrythonExRef
{
    fn new() -> Self where Self: Sized;
    fn get_name(&self) -> &str;
    fn get_id(&self) -> usize;
}

impl_downcast!(TObject);

pub struct Object
{
    pub name: String,
    pub id: usize,
}

impl TObject for Object
{
    fn new() -> Self
    {
        Object
        {
            name: String::new(),
            id: generate_object_id(),
        }
    }

    fn get_name(&self) -> &str
    {
        &self.name
    }

    fn get_id(&self) -> usize
    {
        self.id
    }
}

#[macro_export]
macro_rules! generate_get_name
{
    () =>
    {
        fn get_name(&self) -> &str
        {
            &self.object.name
        }
    };
}

#[macro_export]
macro_rules! generate_get_id
{
    () =>
    {
        fn get_id(&self) -> usize
        {
            self.object.id
        }
    };
}
