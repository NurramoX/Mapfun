// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "geometry"))]
    pub struct Geometry;
}

diesel::table! {
    feeder (id) {
        id -> Int4,
    }
}

diesel::table! {
    lines (from_meter_id, to_meter_id) {
        from_meter_id -> Int4,
        to_meter_id -> Int4,
        feeder_id -> Int4,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Geometry;

    meter (id) {
        id -> Int4,
        position -> Nullable<Geometry>,
        feeder_id -> Nullable<Int4>,
    }
}

diesel::table! {
    spatial_ref_sys (srid) {
        srid -> Int4,
        #[max_length = 256]
        auth_name -> Nullable<Varchar>,
        auth_srid -> Nullable<Int4>,
        #[max_length = 2048]
        srtext -> Nullable<Varchar>,
        #[max_length = 2048]
        proj4text -> Nullable<Varchar>,
    }
}

diesel::joinable!(lines -> feeder (feeder_id));
diesel::joinable!(meter -> feeder (feeder_id));

diesel::allow_tables_to_appear_in_same_query!(
    feeder,
    lines,
    meter,
    spatial_ref_sys,
);
