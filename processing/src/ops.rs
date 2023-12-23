use diesel::prelude::*;
use diesel::{PgConnection, QueryDsl};
use diesel::dsl::{count_star};
use crate::models::{DistanceResult, FeederCount};
use crate::schema::meter;

use crate::models::NewLine; // Assuming you have a models module

pub fn insert_edges_into_lines(connection: &mut PgConnection, edges: Vec<(i32, i32)>, fid: i32) -> Result<usize, diesel::result::Error> {
    use crate::schema::lines::dsl::*;

    let new_lines: Vec<NewLine> = edges.into_iter().map(|(from_id, to_id)| {
        NewLine {
            from_meter_id: from_id,
            to_meter_id: to_id,
            feeder_id: fid
        }
    }).collect();

    diesel::insert_into(lines)
        .values(&new_lines)
        .execute(connection)
}

// Function to execute the query
pub fn get_feeder_counts(conn: &mut PgConnection) -> Result<Vec<FeederCount>, diesel::result::Error> {
    meter::table
        .group_by(meter::feeder_id)
        .select((meter::feeder_id, count_star()))
        .load::<FeederCount>(conn)
}

pub fn get_unique_ids(conn: &mut PgConnection, feeder_id: i32) -> Result<Vec<i32>, diesel::result::Error> {
    meter::table
        .filter(meter::feeder_id.eq(feeder_id)) // Assuming 'feeder_id' is a field in your table
        .select(meter::id) // Assuming 'id' is the field you're interested in
        .load::<i32>(conn) // Loading the results into a Vec<i64>
}

pub fn fetch_distances(connection: &mut PgConnection, feeder_id: i32) -> Result<Vec<DistanceResult>, diesel::result::Error> {
    use diesel::sql_types::{Integer};
    diesel::sql_query("
    WITH filtered_meters AS (SELECT id, position FROM meter WHERE feeder_id = $1)
    SELECT
        m1.id AS point1_id,
        m2.id AS point2_id,
        ST_Distance(m1.position::geography, m2.position::geography) AS distance
    FROM
        filtered_meters m1
    JOIN
        filtered_meters m2
    ON
        m1.id < m2.id
    ")
        .bind::<Integer, _>(feeder_id)
        .load::<DistanceResult>(connection)
}

