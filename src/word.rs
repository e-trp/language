use std::{fs::File, io::{self, BufRead}, path::Path, thread::sleep, time::Duration, env, str::FromStr};
use diesel::{dsl::insert_into, prelude::*, sqlite::SqliteConnection};
use reqwest::blocking;
use scraper::{Html, Selector};
use crate::schema::{verb_forms::dsl::*, words::{dsl::*, id}};
use crate::schema::words;

const DICTIONARY_URL: &str = "https://dictionary.cambridge.org/dictionary/english/";
const DATA_PATH: &str = "data";

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(Path::new(DATA_PATH).join(filename))?;
    Ok(io::BufReader::new(file).lines())
}

pub fn establish_connection() -> SqliteConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::words)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Word {
    pub id: Option<i32>,
    pub source: String,
    pub description: Option<String>,
    pub phonetic: Option<String>,
    pub part_of_speech: Option<String>,
}

#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::verb_forms)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct VerbForms {
    pub id: Option<i32>,
    pub word_id: i32,
    pub base_form: String,
    pub past_simple: String,
    pub past_participle: String,
}

#[derive(Debug, PartialEq, Eq)]
pub struct CommonError;

pub fn get_text(fragment: &Html, selector: &str) -> Result<String, CommonError> {
    Ok(fragment
        .select(&Selector::parse(selector).unwrap())
        .next()
        .unwrap()
        .text()
        .collect::<String>())
}

impl FromStr for Word {
    type Err = CommonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let body = blocking::get(format!("{}{}", DICTIONARY_URL, s)).unwrap();
        let fragment = Html::parse_fragment(&body.text().unwrap());
        let default_string = String::new();
        Ok(Word {
            id: None,
            source: String::from(s),
            description: Some(get_text(&fragment, "div.def.ddef_d.db").unwrap_or(default_string.clone())),
            phonetic: Some(get_text(&fragment, "span.ipa.dipa.lpr-2.lpl-1").unwrap_or(default_string.clone())),
            part_of_speech: Some(get_text(&fragment, "span.pos.dpos").unwrap_or(default_string.clone())),
        })
    }
}
impl  Word {

    pub fn load_irregular_words(filename: &str) {
        let mut conn = establish_connection();
        if let Ok(lines) = read_lines(filename) {
            for line in lines.filter_map(Result::ok) {
                let forms: Vec<&str> = line.split_whitespace().collect();
                if forms.len() >= 3 {
                    if let Ok(new_word) = Word::from_str(forms[0]) {
                        let inserted_word_id= insert_into(words)
                            .values(&new_word)
                            .on_conflict(source)
                            .do_nothing()
                            .returning(id)
                            .get_result::<Option<i32>>(&mut conn)
                            .unwrap();

                        let _ = insert_into(verb_forms).values(
                            VerbForms{
                                id: None,
                                word_id:inserted_word_id.unwrap(), 
                                base_form: forms[0].to_string(), 
                                past_simple:forms[1].to_string(), 
                                past_participle: forms[2].to_string()
                            }
                        ).execute(&mut conn);
                        println!("insert: {new_word:?}");
                    }
                    sleep(Duration::from_secs(3));
                }
            }
        }
    }

    pub fn get_irregular_verb_from_db(target_word: &str) -> Result<Vec<(Word, VerbForms)>, diesel::result::Error>{
        let mut conn  = establish_connection();
        let result = words::table
            .inner_join(verb_forms)
            .filter(words::source.eq(target_word))
            .select((Word::as_select(), VerbForms::as_select()))
            .load::<(Word, VerbForms)>(&mut conn);
        result
    }

}