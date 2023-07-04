use rand::Rng;
use super::caspr::CellestialSphere;


pub enum Question{
    ObjectQuestion {name:String, ra:f32,dec:f32},
    PositionQuestion {ra:f32,dec:f32},
    ThisPointObject {name:String, ra:f32,dec:f32}
}

pub struct GameHandler{
    current_question:usize,
    questions:Vec<Question>,
    used_questions:Vec<usize>
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
                            Some(_name) => {name=String::from(format!("C {}",_name))}
                        }
                    }
                    Some(_name)=>{name=String::from(_name)}
                }
                catalog.push(Question::ObjectQuestion { name: name, ra: deepsky.ra, dec: deepsky.dec })
            }
        }

        Self { current_question: rand::thread_rng().gen_range(0..catalog.len()), questions: catalog,used_questions:Vec::new()}
		

	}

    pub fn next_question(&mut self){

        self.used_questions.push(self.current_question);
        let mut possible_questions:Vec<usize> =Vec::new();
        for question in 0..self.questions.len(){
            if !self.used_questions.contains(&question){
                possible_questions.push(question);
            }
        }
        self.current_question=possible_questions[rand::thread_rng().gen_range(0..possible_questions.len())]

    }
    pub fn get_display_question(&self)-> String{
        match &self.questions[self.current_question]{
            Question::ObjectQuestion{name,ra:_ra,dec:_dec} => {return String::from(format!("Find {}.",name));}
            Question::PositionQuestion { ra:_ra, dec:_dec } => {return String::from("This does not work yet...Sorry :)");}
            Question::ThisPointObject { name:_name, ra:_ra, dec:_dec } => {return  String::from("What is this object?");}
        }
    }
}