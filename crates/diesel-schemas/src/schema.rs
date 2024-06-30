// @generated automatically by Diesel CLI.

diesel::table! {
    bien (id_bien) {
        id_bien -> Uuid,
        nom -> Text,
        region -> Uuid,
        description -> Text,
        proprietaire -> Text,
        type_bien -> Uuid,
    }
}

diesel::table! {
    bien_loyer (id_bien_loyer) {
        id_bien_loyer -> Uuid,
        bien -> Uuid,
        valeur -> Numeric,
        date_entree -> Timestamp,
    }
}

diesel::table! {
    client (email) {
        email -> Text,
        nom -> Text,
    }
}

diesel::table! {
    location (id_location) {
        id_location -> Uuid,
        bien -> Uuid,
        client -> Text,
        date_debut -> Timestamp,
        date_fin -> Nullable<Timestamp>,
    }
}

diesel::table! {
    proprietaire (telephone) {
        telephone -> Text,
        nom -> Text,
    }
}

diesel::table! {
    region (id_region) {
        id_region -> Uuid,
        nom -> Text,
    }
}

diesel::table! {
    type_bien (id_type_bien) {
        id_type_bien -> Uuid,
        designation -> Text,
    }
}

diesel::table! {
    type_bien_commission (id_type_bien_commission) {
        id_type_bien_commission -> Uuid,
        type_bien -> Uuid,
        valeur -> Numeric,
        date_entree -> Timestamp,
    }
}

diesel::joinable!(bien -> proprietaire (proprietaire));
diesel::joinable!(bien -> region (region));
diesel::joinable!(bien -> type_bien (type_bien));
diesel::joinable!(bien_loyer -> bien (bien));
diesel::joinable!(location -> bien (bien));
diesel::joinable!(location -> client (client));
diesel::joinable!(type_bien_commission -> type_bien (type_bien));

diesel::allow_tables_to_appear_in_same_query!(
    bien,
    bien_loyer,
    client,
    location,
    proprietaire,
    region,
    type_bien,
    type_bien_commission,
);
