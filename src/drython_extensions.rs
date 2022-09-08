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

pub trait DrythonExRef
{
    fn get_drython_vars(&self) -> HashMap<String, Token>; 
    fn set_my_vars(&mut self, runner: &mut Runner, identifiers: &str);
}

// DrythonExRef implementations
impl DrythonExRef for Object
{
    fn get_drython_vars(&self) -> HashMap<String, Token>
    {
        HashMap::from([
            ("object.name".to_string(), Token::String(self.get_name().to_string())),
        ])
    }

    fn set_my_vars(&mut self, runner: &mut Runner, identifiers: &str)
    {
        runner.update_variable((format!("{}object.name", identifiers).as_str(), &mut self.name));
    }
}

impl DrythonExRef for Object2D
{
    fn get_drython_vars(&self) -> HashMap<String, drython::types::Token>
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
    fn get_drython_vars(&self) -> std::collections::HashMap<String, drython::types::Token>
    {
        HashMap::from([
            ("transform.pos".to_string(), vector2_to_token(self.pos)),
            ("transform.rot".to_string(), Token::Float(self.rot)),
            ("transform.scale".to_string(), vector2_to_token(self.scale)),
        ])
    }

    fn set_my_vars(&mut self, runner: &mut drython::types::Runner, identifiers: &str)
    {
        runner.update_variable_conversion((format!("{}transform.pos", identifiers).as_str(), &mut self.pos), token_to_vector2);
        runner.update_variable((format!("{}transform.rot", identifiers).as_str(), &mut self.rot));
        runner.update_variable_conversion((format!("{}transform.scale", identifiers).as_str(), &mut self.scale), token_to_vector2);
    }
}
