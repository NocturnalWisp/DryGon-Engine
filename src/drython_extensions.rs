use drython::types::ExFnRef;
use crate::transform::Transform2D;
use crate::object::Object2D;
use std::collections::HashMap;
use drython::types::{Token, Runner};
use raylib::prelude::Vector2;

use crate::object::{Object, TObject};

pub fn vector2_to_token(vec: Vector2) -> Token
{
    Token::Collection(vec![Token::Float(vec.x), Token::Float(vec.y)])
}

pub fn token_to_vector2(token: Token) -> Vector2
{
    if let Token::Collection(c) = token
    {
        if c.len() == 2
        {
            match (&c[0], &c[1])
            {
                (Token::Float(x), Token::Float(y)) => { return Vector2::new(*x,*y); }
                _ => ()
            }
        }
    }

    Vector2::zero()
}

pub type ExVarMap = HashMap<String, Token>;

fn make_ex_var_map(name: &str, value: Token) -> (String, Token)
{
    (name.to_string(), value)
}

pub trait DrythonExRef
{
    fn get_drython_vars(&mut self) -> ExVarMap; 
    fn set_my_vars(&mut self, runner: &mut Runner, identifiers: &str);
}

// DrythonExRef implementations
impl DrythonExRef for Object
{
    fn get_drython_vars(&mut self) -> ExVarMap
    {
        HashMap::from([
            make_ex_var_map("object.name", Token::String(self.get_name())),
        ])
    }

    fn set_my_vars(&mut self, runner: &mut Runner, identifiers: &str)
    {
        runner.update_variable((format!("{}object.name", identifiers).as_str(), &mut self.name));
    }
}

impl DrythonExRef for Object2D
{
    fn get_drython_vars(&mut self) -> ExVarMap
    {
        let mut map = HashMap::from([
        ]);
        map.extend(self.transform.get_drython_vars().into_iter()
            .map(|x| (format!("object.{}", x.0), x.1)));
        map.extend(self.object.get_drython_vars());

        map
    }

    fn set_my_vars(&mut self, runner: &mut Runner, identifiers: &str)
    {
        self.object.set_my_vars(runner, identifiers);
        self.transform.set_my_vars(runner, format!("{}object.", identifiers).as_str());
    }
}

impl DrythonExRef for Transform2D
{
    fn get_drython_vars(&mut self) -> ExVarMap
    {
        HashMap::from([
            make_ex_var_map("transform.pos", vector2_to_token(self.pos)),
            make_ex_var_map("transform.rot", Token::Float(self.rot)),
            make_ex_var_map("transform.scale", vector2_to_token(self.scale)),
        ])
    }

    fn set_my_vars(&mut self, runner: &mut drython::types::Runner, identifiers: &str)
    {
        runner.update_variable_conversion((format!("{}transform.pos", identifiers).as_str(), &mut self.pos), token_to_vector2);
        runner.update_variable((format!("{}transform.rot", identifiers).as_str(), &mut self.rot));
        runner.update_variable_conversion((format!("{}transform.scale", identifiers).as_str(), &mut self.scale), token_to_vector2);
    }
}

// Macro for handling external functions to drython.
macro_rules! create_external_function
{
    ($name: ident, $sub_downcast: tt, $sub_name: ident, $args: ident, $body: block) =>
    {
        
        pub fn $name(optional_object: Option<*mut dyn ExFnRef>, $args: Vec<Token>) -> Result<Option<Token>, String>
        {
            if let Some(object_ref) = optional_object
            {
                unsafe
                {
                    if let Some($sub_name) = (*object_ref.as_mut().unwrap()).as_any_mut().downcast_mut::<Box<dyn TObject>>().unwrap().downcast_mut::<$sub_downcast>()
                    {
                        $body
                    }
                }
            }
            else
            {
                return Err("Could not track optional object.".to_string());
            }

            Ok(None)
        }
    };
}

impl Object
{
    // The object can send of events when an input is registered.
    create_external_function!(register_input, Object2D, object2d, args,
    {
        object2d.transform.pos = Vector2::new(-50.0, 20.0);
        println!("args1: {:?}", args[0]);
    });


    pub fn get_parent()
    {

    }
}
