-- Your SQL goes here
create view v_bien as
SELECT
    ID_BIEN,
    NOM,
    REGION,
    DESCRIPTION,
    PROPRIETAIRE,
    TYPE_BIEN AS ID_TYPE_BIEN,
    V_BIEN_LOYER_RECENT.ID_BIEN_LOYER,
    V_BIEN_LOYER_RECENT.VALEUR AS LOYER,
    V_TYPE_BIEN.ID_TYPE_BIEN_COMMISSION,
    V_TYPE_BIEN.COMMISSION,
    V_TYPE_BIEN.DESIGNATION AS NOM_TYPE_BIEN
FROM
    BIEN
    JOIN V_BIEN_LOYER_RECENT ON V_BIEN_LOYER_RECENT.BIEN = BIEN.ID_BIEN
    JOIN V_TYPE_BIEN ON V_TYPE_BIEN.ID_TYPE_BIEN = BIEN.TYPE_BIEN;