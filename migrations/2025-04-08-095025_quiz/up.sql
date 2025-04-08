-- Your SQL goes here
create table quiz (
    id integer primary key autoincrement,
    name varchar(255) not null unique
);

create table quiz_history (
    id integer primary key autoincrement,
    quiz_id integer not null, 
    result bool not null,
    errors int not null,
    context text not null,
    quiz_date real,
    foreign key(quiz_id) REFERENCES quiz(id)
);