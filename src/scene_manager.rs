use crate::object::TObject;
#[path="scene.rs"]
mod scene;

use yaml_rust::Yaml;
use yaml_rust::YamlLoader;
use crate::Raylib;
use std::fs;

use scene::Scene;
use crate::object::Object2D;
use raylib::prelude::Vector2;

pub struct SceneManager
{
    pub current_scene: Option<Scene>
}

impl SceneManager
{
    pub fn new() -> SceneManager
    {
        SceneManager
        {
            current_scene: None,
        }
    }

    pub fn load(& mut self, raylib: &mut Raylib, scene_path: &str)
    {
        if let Some(scene) = &mut self.current_scene
        {
            scene.unload();
        }

        let mut new_scene = Scene::new(scene_path.to_string());

        match fs::read_to_string(scene_path)
        {
            Ok(file) =>
            {
                match YamlLoader::load_from_str(file.as_str())
                {
                    Ok(mut yaml) =>
                    {
                        // Should only pop a single document, will fail later if the expected
                        // values are not present.
                        if let Some(yaml_doc) = yaml.pop()
                        {
                            self.initialize_scene(raylib, &mut new_scene, &yaml_doc);
                            new_scene.loaded_scene = yaml_doc;
                        }
                        else
                        {
                            println!("Failed to load yaml file: {} due to invalid format.", scene_path);
                        }
                    }
                    Err(error) =>
                    {
                        println!("Failed to load yaml file: {} due to error: {}.", scene_path, error);
                    }
                }
            }
            Err(_) =>
            {
                println!("Failed to find startup file: {}", scene_path);
            }
        }

        self.current_scene = Some(new_scene);
        self.current_scene.as_mut().unwrap();
    }

    fn initialize_scene(&mut self, raylib: &mut Raylib, scene: &mut Scene, unloaded: &Yaml)
    {
        if let Yaml::Hash(hash) = &unloaded["objects 2d"]
        {
            for object in hash
            {
                let mut new_obj = Object2D::new();

                if let Yaml::Hash(params) = object.1
                {
                    for param in params
                    {
                        if let Some(param_name) = param.0.as_str()
                        {
                            match param_name
                            {
                                "name" => { new_obj.object.name = param.1.as_str().unwrap_or("").to_string(); }
                                "sprite" => SceneManager::handle_sprite(&mut new_obj, raylib, param.1),
                                "pos" => { new_obj.transform.pos =
                                    Vector2::new(param.1["x"].as_f64().unwrap_or(0.0) as f32, param.1["y"].as_f64().unwrap_or(0.0) as f32); },
                                "script" => scene.script_manager.handle_script(&new_obj.object, param.1),
                                _ => ()
                            }
                        }
                    }

                }

                scene.objects.push(Box::new(new_obj));
            }

            // Register any script vars.
            scene.script_manager.register_variables(&mut scene.objects);
        }
    }

    fn handle_sprite(new_obj: &mut Object2D, raylib: &mut Raylib, object1: &Yaml)
    {
        if let Some(file_name) = object1.as_str()
        {
            let asset_location: &str = &format!("assets/{}", file_name);
            match raylib.0.load_texture(&raylib.1, asset_location)
            {
                Ok(image) => { new_obj.sprite = Some(image); }
                Err(error) => println!("Failed to load image {} due to {}.", file_name, error)
            }
        }
        else { println!("Invalid sprite file {:?}.", object1); }
    }

}
