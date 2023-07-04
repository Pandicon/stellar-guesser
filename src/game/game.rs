use rand::Rng;
use std::{collections::HashMap, error::Error, fs};

use super::caspr::CellestialSphere;


pub enum Question{
    ObjectQuestion {name:String, ra:f32,dec:f32},
    PositionQuestion {ra:f32,dec:f32}
}

pub struct GameHandler{
    current_question:usize,
    questions:Vec<Question>,
}

impl GameHandler {
    pub fn init(cellestial_sphere:&CellestialSphere)->Self {
        let mut catalog:Vec<Question> = Vec::new();
        for file in cellestial_sphere.deepskies.values(){
            for deepsky in file{

                let name:String;
                match &deepsky.messier{
                    None => {
                        match &deepsky.caldwell{
                            None => {name = String::from("Fuck");},
                            Some(_name) => {name=String::from(_name)}
                        }
                    }
                    Some(_name)=>{name=String::from(_name)}
                }
                catalog.push(Question::ObjectQuestion { name: name, ra: deepsky.ra, dec: deepsky.dec })
            }
        }

        Self { current_question: rand::thread_rng().gen_range(0..catalog.len()), questions: catalog}
		

	}

    pub fn next_question(&mut self){
        self.current_question = rand::thread_rng().gen_range(0..self.questions.len())

    }
    pub fn get_display_question(&self)-> String{
        match &self.questions[self.current_question]{
            Question::ObjectQuestion{name,ra:_ra,dec:_dec} => {return String::from(format!("Find {}.",name));}
            Question::PositionQuestion { ra:_ra, dec:_dec } => {return String::from("This does not work yet...");}
        }
    }
}