-- Your SQL goes here'
create table service_card (
    id serial primary key,
    name varchar not null,
    url varchar not null,
    active boolean not null default 't'
)
