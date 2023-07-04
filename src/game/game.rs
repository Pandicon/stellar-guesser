
pub enum Question{
    PositionQuestion {name:String, ra:f32,dec:f32}
}

pub struct GameHandler{
    current_question:f32,
    questions:Vec<Question>,
    used_questions:Vec<f32>
}

impl GameHandler {
    pub fn init()-> Self{
        



        Self { current_question: 0.0, questions: Vec::new(), used_questions: Vec::new() }
    }
    pub fn next_question(&self){
        println!("Yo mama fat. ");
    }
    pub fn get_display_question(&self) -> String{
        String::from( "Yo mama fat 'n gae. ")
    }
}