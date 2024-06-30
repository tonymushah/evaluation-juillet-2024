-- Your SQL goes here
create view v_bien_loyer_recent as WITH ranked_loyer AS (
    SELECT
        id_bien_loyer,
        bien,
        valeur,
        date_entree,
        ROW_NUMBER() OVER (
            PARTITION BY bien
            ORDER BY
                date_entree DESC
        ) AS rn
    FROM
        bien_loyer
)
SELECT
    id_bien_loyer,
    bien,
    valeur,
    date_entree
FROM
    ranked_loyer
WHERE
    rn = 1;