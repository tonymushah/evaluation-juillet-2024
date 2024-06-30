-- Your SQL goes here
create view v_location_recent as WITH ranked_location AS (
    SELECT
        id_location,
        bien,
        client,
        date_debut,
        date_fin,
        ROW_NUMBER() OVER (
            PARTITION BY bien
            ORDER BY
                date_debut DESC
        ) AS rn
    FROM
        location
)
SELECT
    id_location,
    bien,
    client,
    date_debut,
    date_fin
FROM
    ranked_location
WHERE
    rn = 1;