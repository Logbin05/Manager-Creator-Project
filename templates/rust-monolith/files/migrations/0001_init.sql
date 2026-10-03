create table if not exists users (
    id         uuid        primary key,
    email      text        not null unique,
    name       text        not null,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now()
);

create index if not exists users_created_at_idx on users (created_at desc);
