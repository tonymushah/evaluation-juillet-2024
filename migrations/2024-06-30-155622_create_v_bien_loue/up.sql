-- Your SQL goes here
create view v_bien_loue as
select
    v_location_recent.bien as bien
from
    v_location_recent
WHERE
    v_location_recent.date_debut is not NULL
    and v_location_recent.date_fin is NULL
group by
    v_location_recent.bien;