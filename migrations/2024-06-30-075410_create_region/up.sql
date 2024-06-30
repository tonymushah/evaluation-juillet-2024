-- Your SQL goes here
CREATE table region (
    id_region UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    nom TEXT NOT NULL
);