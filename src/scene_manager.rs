use crate::Raylib;
use std::fs;

use crate::object::Object2D;

use super::object::Object;
use drython::types::{Parser, Runner};
use yaml_rust::{YamlLoader, Yaml};

use raylib::prelude::Vector2;

pub struct SceneManager<'a>
{
    pub current_scene: Option<Scene<'a>>
}

impl<'a> SceneManager<'a>
{
    pub fn new() -> SceneManager<'a>
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

        let mut new_scene = Scene
        {
            scene_path: scene_path.to_string(),
            loaded_scene: Yaml::BadValue,
            objects2d: vec![]
        };

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
                                "script" => SceneManager::handle_script(&mut new_obj.object, param.1),
                                _ => ()
                            }
                        }
                    }
                }

                scene.objects2d.push(new_obj);
            }
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

    fn handle_script(new_obj: &mut Object, script_path: &Yaml)
    {
        if let Some(file_name) = script_path.as_str()
        {
            if let Ok(canon) = std::fs::canonicalize(format!("assets/{}", file_name))
            {
                if let Some(full_path) = canon.to_str()
                {
                    match Parser::parse_file(full_path, &mut new_obj.script_errors)
                    {
                        Ok(parser) => { new_obj.script_path = file_name.to_string(); new_obj.script = Some(Runner::new(parser));
                        }
                        Err(error) => println!("Failed to load script {} due to {}.", file_name, error)
                    }
                }
                else
                {
                    println!("Failed to load script from {:?}. Path contains non-unicode characters.", canon);
                }
            }
            else
            {
                println!("Failed to load script from {:?}. Path is not local to asset folder.", file_name);
            }
        }
        else { println!("Invalid sprite file {:?}.", script_path); }
    }
}

pub struct Scene<'a>
{
    pub scene_path: String,
    loaded_scene: Yaml,
    pub objects2d: Vec<Object2D<'a>>
}

impl<'a> Scene<'a>
{
    pub fn unload(&mut self)
    {
        
    }
}
