pub mod scene_manager;
pub mod object;
pub mod transform;

use drython::types::Token;
use drython::types::error::ErrorManager;
use raylib::RaylibHandle;
use raylib::RaylibThread;
use std::fs;
use std::time::Duration;
use std::time::SystemTime;

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

        let mut error_manager = ErrorManager::new();

        if let Some(current_scene) = &mut self.scene_manager.current_scene
        {
            for object in &mut current_scene.objects2d
            {
                if let Some(runner) = &mut object.object.script
                {
                    runner.run_setup(&mut error_manager);
                    runner.call_function("start", vec![], &mut error_manager);
                    // println!("{:#?}", error_manager.errors.iter().map(|x| x.display()).collect::<Vec<String>>());
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
                    for object in &mut current_scene.objects2d
                    {
                        if let Some(runner) = &mut object.object.script
                        {
                            runner.call_function("update", vec![Token::Float(game_delta_time)], &mut error_manager);
                        }

                    }
                }
                accumulator -= target_frame_time;
            }

            let mut d = rl.begin_drawing(&thread);

            d.clear_background(Color::WHITE);

            // 2d object drawing.
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
            raylib.0.set_window_title(raylib.1, name);
        }

        if let Yaml::String(main_scene) = &contents[0]["main_scene"]
        {
            self.main_scene_path = main_scene.to_string();
            self.scene_manager.load(raylib, &self.main_scene_path);
        }
    }
}
