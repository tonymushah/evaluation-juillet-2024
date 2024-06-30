-- Your SQL goes here
create table bien_loyer(
    id_bien_loyer UUID PRIMARY KEY DEFault gen_random_uuid(),
    bien UUID NOT NULL REFERENCES bien(id_bien),
    VALEUR NUMERIC NOT NULL,
    date_entree TIMESTAMP NOT NULL DEFault now()
);