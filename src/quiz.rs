use diesel::prelude::*;
use crate::word::{VerbForms, Word};
use rand::prelude::*;
use log::debug;


#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = crate::schema::quiz)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Quiz {
    pub id: Option<i32>,
    pub name: String,
}


#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
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


#[derive(Debug, Clone)]
pub struct IrregVerbQuiz {
    pub quiz: Quiz,
    pub words: Vec<VerbForms>,
    pub rand_indexes: Vec<usize>
}


impl IrregVerbQuiz {

    pub fn new() -> Self {
        let words = Word::fetch_irregular_verbs().unwrap();
        let mut rand_indexes = (0..words.len()).collect::<Vec<usize>>();
        rand_indexes.shuffle(&mut rand::rng());
        let quiz = Quiz{ id: None, name: "irregular verb".to_string()};
        Self{quiz, words, rand_indexes}
    }

    pub fn next_word(&mut self) -> Option<VerbForms> {
        let random_index = self.rand_indexes.pop();
        match random_index {None => None, Some(i) => Some(self.words[i].clone())}
    }


}