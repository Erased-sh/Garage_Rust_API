-- Your SQL goes here
create table cvs (
    id serial primary key,
    title text not null,
    body text not null,
    author text not null
)