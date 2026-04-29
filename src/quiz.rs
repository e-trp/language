use diesel::{prelude::*, result::Error};
use crate::{
    schema,
    word::{establish_connection, log_query, VerbForms, Word},
};
use rand::seq::SliceRandom;
use chrono::Local;

const IRREGULAR_VERB_QUIZ: i32 = 1;
#[allow(dead_code)]
const FULL_TEXT_QUIZ: i32 = 2;

#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = schema::quiz)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Quiz {
    pub id: Option<i32>,
    pub name: String,
}

#[derive(Debug, Queryable, Selectable, Insertable, Clone)]
#[diesel(table_name = schema::quiz_history)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct QuizHistory {
    pub id: Option<i32>,
    pub quiz_id: i32,
    pub result: bool,
    pub errors: i32,
    pub context: String,
    pub quiz_date: Option<f64>,
}

pub enum QuizType {
    IrregVerb,
    Text,
}

#[derive(Debug, Clone)]
pub struct IrregVerbQuiz {
    pub quiz: Quiz,
    pub words: Vec<VerbForms>,
    pub rand_indexes: Vec<usize>,
    pub wrong_words: Vec<VerbForms>,
}

impl Default for IrregVerbQuiz {
    fn default() -> Self {
        Self::new()
    }
}

impl IrregVerbQuiz {
    pub fn new() -> Self {
        let words = Word::fetch_irregular_verbs().unwrap();
        let len = words.len();

        let mut rand_indexes: Vec<_> = (0..len).collect();
        rand_indexes.shuffle(&mut rand::rng());

        let quiz = Quiz {
            id: Some(IRREGULAR_VERB_QUIZ),
            name: "irregular verb".to_string(),
        };

        Self {
            quiz,
            words,
            rand_indexes,
            wrong_words: Vec::with_capacity(len),
        }
    }

    pub fn next_word(&mut self) -> Option<VerbForms> {
        self.rand_indexes
            .pop()
            .map(|i| self.words[i].clone())
    }

    pub fn add_wrong_word(&mut self, word: VerbForms) {
        self.wrong_words.push(word);
    }

    pub fn write_record(&self) -> Result<usize, Error> {
        let mut conn = establish_connection();

        let history = QuizHistory {
            id: None,
            quiz_id: self.quiz.id.unwrap(),
            result: self.wrong_words.is_empty(),
            errors: self.wrong_words.len() as i32,
            quiz_date: Some(Local::now().timestamp() as f64),
            context: self.generate_errors_data(),
        };

        let query = diesel::insert_into(schema::quiz_history::table).values(&history);
        log_query(&query);

        query.execute(&mut conn)
    }

    pub fn generate_errors_data(&self) -> String {
        self.wrong_words
            .iter()
            .map(|item| {
                format!(
                    "id:{}, base_form:{}\n",
                    item.id.unwrap(),
                    item.base_form
                )
            })
            .collect()
    }
}