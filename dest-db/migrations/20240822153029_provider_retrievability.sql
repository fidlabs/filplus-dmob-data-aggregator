-- Add migration script here
create table provider_retrievability (
    provider text not null primary key,
    total bigint not null,
    successful bigint not null,
    success_rate float8 not null
);