-- Your SQL goes here
create table bien (
    id_bien UUID PRIMARY KEY DEFault gen_random_uuid(),
    nom TEXT NOT NULL,
    region UUID REFERENCES region(id_region) NOT NULL,
    description TEXT NOT NULL,
    proprietaire TEXT NOT NULL REFERENCES proprietaire(telephone),
    type_bien UUID REFERENCES type_bien(ID_TYPE_BIEN) NOT NULL
);