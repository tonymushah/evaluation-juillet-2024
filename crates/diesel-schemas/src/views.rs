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
