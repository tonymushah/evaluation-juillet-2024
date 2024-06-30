-- Your SQL goes here
create view v_type_bien_commission_recent as 
WITH ranked_commissions AS (
    SELECT 
        id_type_bien_commission,
        type_bien,
        valeur,
        date_entree,
        ROW_NUMBER() OVER (PARTITION BY type_bien ORDER BY date_entree DESC) AS rn
    FROM type_bien_commission
)
SELECT 
    id_type_bien_commission,
    type_bien,
    valeur,
    date_entree
FROM ranked_commissions
WHERE rn = 1;