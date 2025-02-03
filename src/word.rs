use std::{fs::File, io::{self, BufRead}, path::Path, thread::sleep, time::Duration, env, str::FromStr};
use diesel::{prelude::*, dsl::insert_into, sqlite::SqliteConnection};
use dotenv::dotenv;
use reqwest::blocking;
use scraper::{Html, Selector};
use crate::schema::words::dsl::*;

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
    dotenv().ok();
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

pub fn load_words(filename: &str) {
    let mut connection = establish_connection();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.filter_map(Result::ok) {
            let forms: Vec<&str> = line.split_whitespace().collect();
            if let Ok(new_word) = Word::from_str(forms[0]) {
                let _ = insert_into(words)
                    .values(&new_word)
                    .on_conflict(source)
                    .do_nothing()
                    .returning(id)
                    .execute(&mut connection);
                println!("{:?}", new_word);
            }
            sleep(Duration::from_secs(1));
        }
    }
}