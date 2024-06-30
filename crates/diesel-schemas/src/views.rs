diesel::table! {
    v_type_bien_commission_recent (id_type_bien_commission) {
        id_type_bien_commission -> Uuid,
        type_bien -> Uuid,
        valeur -> Numeric,
        date_entree -> Timestamp,
    }
}

diesel::table! {
    v_type_bien(id_type_bien, id_type_bien_commission) {
        id_type_bien -> Uuid,
        designation -> Text,
        id_type_bien_commission -> Uuid,
        commission -> Numeric
    }
}

diesel::table! {
    v_bien_loyer_recent (id_bien_loyer) {
        id_bien_loyer -> Uuid,
        bien -> Uuid,
        valeur -> Numeric,
        date_entree -> Timestamp,
    }
}

diesel::table! {
    v_bien (id_bien, id_type_bien, proprietaire, id_type_bien_commission) {
        id_bien -> Uuid,
        nom -> Text,
        region -> Uuid,
        description -> Text,
        proprietaire -> Text,
        id_type_bien -> Uuid,
        loyer -> Numeric,
        id_type_bien_commission -> Uuid,
        commission -> Numeric,
        nom_type_bien -> Text
    }
}

diesel::table! {
    v_location_recent(id_location) {
        id_location -> Uuid,
        bien -> Uuid,
        client -> Text,
        date_debut -> Timestamp,
        date_fin -> Nullable<Timestamp>,
    }
}

diesel::table! {
    v_bien_non_loue(bien) {
        bien -> Uuid
    }
}

diesel::table! {
    v_bien_non_loue(bien) {
        bien -> Uuid
    }
}
