-- Your SQL goes here
create table payement (
    id_payement UUID PRIMARY KEY default gen_random_uuid(),
    location UUID REFERENCES location(id_location) NOT NULL,
    montant decimal not null,
    date_paiement TIMESTAMP not null default now(),
    loyer UUID references bien_loyer(id_bien_loyer) not null,
    commission_id UUID REFERENCES type_bien_commission(id_type_bien_commission) NOT NULL
)