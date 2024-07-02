-- Your SQL goes here
create table location_speculative (
    id_location_speculative UUID PRIMARY KEY default gen_random_uuid(),
    location UUID Not NULL references location(id_location),
    date_ref DATE NOT NULL,
    loyer_initial DECIMAL not null,
    commission DECIMAL Not null,
    num_mois int Not NULL,
    loyer_a_payer DECIMAL not NULL,
    valeur_commission DECIMAL not null
);