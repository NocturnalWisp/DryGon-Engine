use downcast_rs::Downcast;

use drython::types::ExFnRef;
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

pub trait TObject: Downcast + DrythonExRef + ExFnRef
{
    fn new() -> Self where Self: Sized;
    fn get_name(&self) -> String;
    fn get_id(&self) -> usize;

    fn get_obj(&mut self) -> &mut Object;
}

impl_downcast!(TObject);

pub struct Object
{
    pub name: String,
    pub id: usize,

    inputs: Vec<(String, String)>,
}

impl Object
{
    pub fn set_name(&mut self, new_name: &str)
    {
        self.name = new_name.to_string();
    }

    pub fn set_name2(&mut self, new_name: String)
    {
        self.name = new_name;
    }
}

impl TObject for Object
{
    fn new() -> Self
    {
        Object
        {
            name: String::new(),
            id: generate_object_id(),

            inputs: Vec::new(),
        }
    }

    fn get_name(&self) -> String
    {
        self.name.clone()
    }

    fn get_id(&self) -> usize
    {
        self.id
    }

    fn get_obj(&mut self) -> &mut Object
    {
        self
    }
}

impl ExFnRef for Box<dyn TObject>
{
    fn as_any(&self) -> &dyn std::any::Any {self}
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
}
impl ExFnRef for Object
{
    fn as_any(&self) -> &dyn std::any::Any {self}
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
}
impl ExFnRef for Object2D
{
    fn as_any(&self) -> &dyn std::any::Any {self}
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {self}
}

#[macro_export]
macro_rules! generate_get_name
{
    () =>
    {
        fn get_name(&self) -> String
        {
            self.object.name.clone()
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
