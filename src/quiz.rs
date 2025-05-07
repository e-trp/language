use diesel::{dsl::insert_into, prelude::*, result::Error};
use crate::{schema::{self, quiz_history::{self, errors}}, word::{self, establish_connection, VerbForms, Word}};
use rand::prelude::*;
use chrono::Local;


const IRREGULAR_VERB_QUIZ: i32 = 1;
const FULL_TEXT_QUIZ: i32 = 2;

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
    pub quiz_errors_data: Option<String>,
    pub quiz_date: Option<f64>
}


pub enum QuizType {
    IrregVerb,
    Text
}


#[derive(Debug, Clone)]
pub struct IrregVerbQuiz {
    pub quiz: Quiz,
    pub words: Vec<VerbForms>,
    pub rand_indexes: Vec<usize>,
    pub wrong_words: Vec<VerbForms>
}


impl IrregVerbQuiz {

    pub fn new() -> Self {
        let words = Word::fetch_irregular_verbs().unwrap();
        let len = words.len();
        let mut rand_indexes = (0..words.len()).collect::<Vec<usize>>();
        rand_indexes.shuffle(&mut rand::rng());
        let quiz = Quiz{ id: Some(IRREGULAR_VERB_QUIZ), name: "irregular verb".to_string()};
        Self{quiz, words, rand_indexes, wrong_words: Vec::<VerbForms>::with_capacity(len)}
    }

    pub fn next_word(&mut self) -> Option<VerbForms> {
        let random_index = self.rand_indexes.pop();
        match random_index {None => None, Some(i) => Some(self.words[i].clone())}
    }

    pub fn add_wrong_word(&mut self, word: VerbForms)  {
        self.wrong_words.push(word);
    }

    pub fn write_record(&self)  -> Result<usize, Error>  {
        let mut conn = establish_connection();
        insert_into(schema::quiz_history::table).values(QuizHistory{
            id: None, quiz_id: self.quiz.id.unwrap(),
            result: self.wrong_words.len() == 0, 
            errors: self.wrong_words.len() as i32,
            quiz_date: Some(Local::now().timestamp() as f64),
            quiz_errors_data: Some(self.generate_errors_data())
        }).execute(&mut conn)
    }

    pub fn generate_errors_data(&self) -> String{
        let mut result: String = String::with_capacity(self.wrong_words.len() * 50);
        for item in &self.wrong_words {
            let line = format!("id:{}, base_form:{}\n", item.id.unwrap(), item.base_form);
            result.push_str(&line);
        }
        result
    }

}