use diesel::prelude::*;
use crate::word::Word;
use rand::prelude::*;


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::quiz)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Quiz {
    pub id: Option<i32>,
    pub name: String,
}


#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::quiz_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct QuizHistory {
    pub id: Option<i32>,
    pub quiz_id: i32,
    pub result: bool,
    pub errors: i32,
    pub quiz_date: Option<f32>
}


pub enum QuizType {
    IrregVerb,
    Text
}


impl  Quiz {
    
    pub fn new(name: &str) -> Self {
        Self{id: None, name: name.to_string()}
    }

    pub fn run_quiz(&self) {
        match self.name.as_str() {
            "irregular_verbs" => {
                if let Ok(words)= Word::fetch_irregular_verbs() {
                    let mut shuffle_indexes = (0..words.len()).collect::<Vec<usize>>();
                    shuffle_indexes.shuffle(&mut rand::rng());
                    for i in shuffle_indexes {
                        println!("{:?}", words[i])
                    }
                }

            },
            "text" => {panic!("not implemented")}
            _ => {panic!("not implemented")}
        }
    }


}