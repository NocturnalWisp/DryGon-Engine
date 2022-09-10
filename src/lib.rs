#![feature(trait_upcasting)]
#![feature(thread_local)]
#![allow(incomplete_features)]

#[macro_use]
extern crate downcast_rs;

pub mod scene_manager;
pub mod object;
pub mod transform;

mod drython_extensions;

use crate::object::Object2D;
use drython::types::Token;
use raylib::RaylibHandle;
use raylib::RaylibThread;
use std::fs;
use std::time::Duration;
use std::time::SystemTime;

use raylib::prelude::*;

extern crate yaml_rust;

use scene_manager::SceneManager;
use yaml_rust::{YamlLoader, Yaml};

pub struct Game
{
    name: String,
    main_scene_path: String,
    scene_manager: SceneManager,
}

type Raylib<'a> = (&'a mut RaylibHandle, &'a RaylibThread);

impl Game
{

    pub fn new(main_yaml_path: &str) -> Game
    {
        Game
        {
            name: String::new(),
            main_scene_path: main_yaml_path.to_string(),
            scene_manager: SceneManager::new(),
        }
    }

    pub fn start(&mut self) -> &mut Self
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

        if let Some(current_scene) = &mut self.scene_manager.current_scene
        {
            current_scene.script_manager.run_setup();
            current_scene.script_manager.run_function_all("start", None);
            current_scene.script_manager.update_variables(&mut current_scene.objects);

            for object in &current_scene.objects
            {
                if let Some(object2d) = object.downcast_ref::<Object2D>()
                {
                    println!("After Change: {:?}", object2d.object.name);
                }
            }
        }

        // Game Loop
        let mut last_update_time = SystemTime::now();
        let game_time_factor: f32 = 1.0;
        // 33.3 milliseconds.
        let target_frame_time = Duration::from_nanos(33_300_000);
        let mut accumulator = Duration::new(0, 0);

        while !rl.window_should_close()
        {
            let now = SystemTime::now();
            let real_delta_time = now.duration_since(last_update_time).unwrap_or(Duration::new(0, 0));
            last_update_time += real_delta_time;
            accumulator += real_delta_time;
            let game_delta_time = real_delta_time.as_secs_f32() * game_time_factor;

            // Handle frame rate based UPDATING.
            while accumulator > target_frame_time
            {
                if let Some(current_scene) = &mut self.scene_manager.current_scene
                {
                    current_scene.script_manager.run_function_all("update", Some(vec![Token::Float(game_delta_time)]));
                    current_scene.script_manager.update_variables(&mut current_scene.objects);
                }
                accumulator -= target_frame_time;
            }

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);

            // 2d object drawing.
            if let Some(current_scene) = &self.scene_manager.current_scene
            {
                for object in &current_scene.objects
                {
                    if let Some(object2d) = object.downcast_ref::<Object2D>()
                    {
                        if let Some(texture) = &object2d.sprite
                        {
                            let pos = object2d.transform.pos;
                            d.draw_texture(texture, pos.x as i32, pos.y as i32, raylib::color::Color::WHITE); 
                        }
                    }
                }
            }
        }

        self
    }

    fn startup_yaml(&mut self, raylib: &mut Raylib, contents: Vec<Yaml>)
    {
        if let Yaml::String(name) = &contents[0]["name"]
        {
            self.name = name.to_string();
            raylib.0.set_window_title(raylib.1, name);
        }

        if let Yaml::String(main_scene) = &contents[0]["main_scene"]
        {
            self.main_scene_path = format!("assets/{}", main_scene);
            self.scene_manager.load(raylib, &self.main_scene_path);
        }
    }
}
