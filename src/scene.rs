use crate::object::TObject;
use crate::scene_manager::scene::script_manager_mod::ScriptManager;

use yaml_rust::Yaml;

#[path="script_manager.rs"]
mod script_manager_mod;

pub struct Scene
{
    pub scene_path: String,
    pub loaded_scene: Yaml,
    pub objects: Vec<Box<dyn TObject>>,
    pub script_manager: script_manager_mod::ScriptManager
}

impl Scene
{
    pub fn new(scene_path: String) -> Self
    {
        Scene
        {
            scene_path,
            loaded_scene: Yaml::BadValue,
            objects: vec![],
            script_manager: ScriptManager::new(),
        }
    }

    pub fn unload(&mut self)
    {
        
    }
}
