-- Your SQL goes here
create table if not exists verb_forms (
    id integer primary key autoincrement,
    word_id integer not null,
    base_form varchar(100) not null,
    past_simple varchar(100) not null, 
    past_participle varchar(100) not null,
    foreign key(word_id) REFERENCES words(id)
);