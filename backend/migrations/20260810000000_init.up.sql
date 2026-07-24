create table if not exists items (
    id uuid primary key,
    name varchar(128) not null,
    description varchar(1024),
    created_at timestamp with time zone not null,
    updated_at timestamp with time zone not null
);

create index if not exists idx_items_created_at on items (created_at desc);
