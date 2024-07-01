-- Your SQL goes here
create view v_payement_grouped_extended as
SELECT
    PG.LOCATION,
    PG.MONTANT,
    pg.loyer as loyer_id,
    bien_loyer.valeur - pg.montant as reste,
    (pg.montant * type_bien_commission.valeur) / 100 as valeur_commission,
    pg.commission_id
fROM
    V_PAYEMENT_GROUPED AS PG
    JOIN bien_loyer on bien_loyer.id_bien_loyer = pg.loyer
    JOIN type_bien_commission on type_bien_commission.id_type_bien_commission = pg.commission_id;