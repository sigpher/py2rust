create table if not exists films(
    id integer primary key autoincrement,
    name text not null,
    alias text not null,
    cover text not null,
    country text not null,
    length text not null,
    published_at text not null,
    drama text not null,
    score text not null);
