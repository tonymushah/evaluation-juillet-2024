-- Your SQL goes here
create table type_bien(
    id_type_bien UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    designation TEXT NOT NULL
);