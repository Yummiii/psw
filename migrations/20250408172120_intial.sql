-- Add migration script here
create table wallpapers (
    id integer primary key autoincrement,
    "key" varchar(255) not null,
    name varchar(255) not null,
    data longblob not null,
    current boolean not null default false
);
