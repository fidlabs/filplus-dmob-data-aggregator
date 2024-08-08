-- Add migration script here
create table providers (
    provider text not null primary key,
    first_client text not null
);