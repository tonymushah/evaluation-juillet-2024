-- Your SQL goes here
create view v_bien_non_loue as
select
    v_location_recent.bien as bien
from
    v_location_recent
    right join bien on v_location_recent.bien = bien.id_bien
WHERE
    (
        v_location_recent.date_fin is not NULL
        and v_location_recent.date_fin < now()
    )
    or (
        v_location_recent.date_debut is null
        and v_location_recent.date_fin is null
    )
group by
    v_location_recent.bien;