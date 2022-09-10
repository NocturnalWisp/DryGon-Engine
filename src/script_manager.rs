use crate::object::{Object, TObject};
use std::collections::HashMap;
use drython::types::Parser;
use yaml_rust::Yaml;

use drython::types::error::ErrorManager;
use drython::types::{Runner, Token};

pub type Script = (String, Runner, ErrorManager);

pub struct ScriptManager
{
    pub scripts: HashMap<usize, Script>,
}

impl ScriptManager
{
    pub fn new() -> Self
    {
        ScriptManager
        {
            scripts: HashMap::new(),
        }
    }

    pub fn run_setup(&mut self)
    {
        for script in &mut self.scripts
        {
            script.1.1.run_setup(&mut script.1.2);
            println!("{:#?}", script.1.1.parser.global_expressions);
        }
    }

    // Adds a new script to the list from a .dry file.
    pub fn handle_script(& mut self, new_obj: &Object, script_path: &Yaml)
    {
        if let Some(file_name) = script_path.as_str()
        {
            if let Ok(canon) = std::fs::canonicalize(format!("assets/{}", file_name))
            {
                if let Some(full_path) = canon.to_str()
                {
                    let mut error_manager = ErrorManager::new();

                    match Parser::parse_file(full_path, &mut error_manager)
                    {
                        Ok(parser) => 
                        {
                            if error_manager.errors.len() == 0
                            {
                                self.scripts.insert(new_obj.get_id(), (full_path.to_string(), Runner::new(parser), error_manager));
                            }
                            else
                            {
                                println!("Script parsed with errors. Please fix them before your script becomes active:\n{:#?}",
                                         error_manager.errors);
                            }
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

    pub fn register_externals(&mut self, objects: &mut Vec<Box<dyn TObject>>)
    {
        for object in objects.iter_mut()
        {
            if self.scripts.contains_key(&object.get_id())
            {
                // Variables
                let obj_map = object.get_drython_vars();

                // Functions.
                
                self.scripts.entry(object.get_id())
                    .and_modify(|x|
                    {
                        x.1.register_variables(obj_map);
                        x.1.register_external_function("register_input", Some(object), Box::new(Object::register_input));
                    }
                );
            }

        }
    }

    pub fn update_variables<T>(&mut self, objects: &mut Vec<Box<T>>)
        where
            T: TObject,
            T: ?Sized
    {
        for object in objects
        {
            if self.scripts.contains_key(&object.get_id())
            {
                self.scripts.entry(object.get_id()).and_modify(|x| object.set_my_vars(&mut x.1, ""));
            }
        }
    }

    pub fn run_function_all(&mut self, name: &str, args: Option<Vec<Token>>)
    {
        let mut delete_me = None;
        for script in &mut self.scripts
        {
            script.1.1.call_function(name, args.clone().unwrap_or(vec![]), &mut script.1.2);

            if script.1.2.errors.len() > 0
            {
                println!("{} failed to call {}, due to the following errors:\n{:#?}\nWarning: Script has been disabled.",
                         script.1.0, name, script.1.2.errors);

                delete_me = Some(*script.0);
            }
        }

        if let Some(delete) = delete_me
        {
            self.scripts.remove(&delete);
        }
    }
}
