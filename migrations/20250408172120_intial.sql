create table wallpapers (
    id integer primary key autoincrement,
    "key" varchar(255) not null unique,
    data longblob not null,
    name varchar(255) not null
);
