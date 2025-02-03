-- Your SQL goes here
create table if not exists words (
    id integer primary key autoincrement,
    source varchar(255) unique not null,
    description varchar(500),
    phonetic varchar(100),
    part_of_speech varchar(100)
);