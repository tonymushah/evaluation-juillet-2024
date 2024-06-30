-- Your SQL goes here
create table type_bien_commission (
    id_type_bien_commission UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_bien UUID REFERENCES type_bien(id_type_bien) NOT NULL,
    valeur decimal NOT NULL,
    date_entree TIMESTAMP NOT NULL DEFAULT now()
);