pub mod scene_manager;
pub mod object;
pub mod transform;

use raylib::RaylibHandle;
use raylib::RaylibThread;
use std::fs;

use raylib::prelude::*;

extern crate yaml_rust;

use scene_manager::SceneManager;
use yaml_rust::{YamlLoader, Yaml};

pub struct Game<'a>
{
    name: String,
    main_scene_path: String,
    scene_manager: SceneManager<'a>,
}

type Raylib<'a> = (&'a mut RaylibHandle, &'a RaylibThread);

impl<'a> Game<'a>
{
    pub fn new(main_yaml_path: &str) -> Game<'a>
    {
        Game
        {
            name: String::new(),
            main_scene_path: main_yaml_path.to_string(),
            scene_manager: SceneManager::new(),
        }
    }

    pub fn start(&mut self)
    {
        let (mut rl, thread) = raylib::init()
            .size(640, 480)
            .title(&self.name)
            .build();

        match fs::read_to_string(&self.main_scene_path)
        {
            Ok(file) =>
            {
                match YamlLoader::load_from_str(file.as_str())
                {
                    Ok(yaml) =>
                    {
                        self.startup_yaml(&mut (&mut rl, &thread), yaml);
                    }
                    Err(error) =>
                    {
                        println!("Failed to load yaml file: {} due to error: {}.", self.main_scene_path, error);
                    }
                }
            }
            Err(error) =>
            {
                println!("Failed to find startup file: {} due to: {}.", self.main_scene_path, error);
            }
        }
        

        while !rl.window_should_close() {
            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);
            d.draw_text("Hello, world!", 12, 12, 20, Color::BLACK);

            if let Some(current_scene) = &self.scene_manager.current_scene
            {
                for object in &current_scene.objects2d
                {
                    if let Some(texture) = &object.sprite
                    {
                        let pos = object.transform.pos;
                        d.draw_texture(texture, pos.x as i32, pos.y as i32, raylib::color::Color::WHITE); 
                    }
                }
            }
        }
    }

    fn startup_yaml(&mut self, raylib: &mut Raylib, contents: Vec<Yaml>)
    {
        if let Yaml::String(name) = &contents[0]["name"]
        {
            self.name = name.to_string();
        }

        if let Yaml::String(main_scene) = &contents[0]["main_scene"]
        {
            self.main_scene_path = main_scene.to_string();
            self.scene_manager.load(raylib, &self.main_scene_path);
        }
    }
}
