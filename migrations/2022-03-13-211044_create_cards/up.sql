create table category (
    id serial primary key,
    name varchar not null
);

create table service (
    id serial primary key,
    name varchar not null,
    url varchar not null,
    active boolean not null default 't'
);

create table person (
    id serial primary key,
    name varchar not null
);

create table service_categories (
    service_id serial references service(id),
    category_id serial references category(id),
    primary key (service_id, category_id)
);

create table user_categories (
    person_id serial references person(id),
    category_id serial references category(id),
    primary key (person_id, category_id)
);
