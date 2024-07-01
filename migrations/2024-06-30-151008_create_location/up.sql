-- Your SQL goes here
create table location (
    id_location UUID PRIMARY KEY default gen_random_uuid(),
    bien TEXT REFERENCES bien(id_bien) NOT null,
    client TEXT REFERENCES client(email) NOT null,
    date_debut TIMESTAMP NOT NULL default now(),
    date_fin TIMESTAMP
);