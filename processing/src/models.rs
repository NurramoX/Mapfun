use diesel::{Insertable, Queryable, QueryableByName};
use diesel::sql_types::{Integer, Double};

use crate::schema::lines; // Assuming you have a schema module

#[derive(Insertable)]
#[table_name="lines"]
pub struct NewLine {
    pub from_meter_id: i32,
    pub to_meter_id: i32,
    pub feeder_id: i32,
}


#[derive(QueryableByName, Debug, PartialEq)]
pub struct DistanceResult {
    #[diesel(sql_type = Integer)]
    #[diesel(column_name = point1_id)]
    pub point1_id: i32,

    #[diesel(sql_type = Integer)]
    #[diesel(column_name = point2_id)]
    pub point2_id: i32,

    #[diesel(sql_type = Double)]
    #[diesel(column_name = distance)]
    pub distance: f64,
}

// Define a struct to hold the query result
#[derive(Queryable, Debug)]
pub struct FeederCount {
    pub feeder_id: Option<i32>,
    pub count: i64,
}
