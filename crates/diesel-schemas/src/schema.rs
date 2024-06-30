// @generated automatically by Diesel CLI.

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

diesel::allow_tables_to_appear_in_same_query!(
    proprietaire,
    region,
    type_bien,
);
