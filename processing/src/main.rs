use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::{env};
mod ops;
mod models;
mod schema;
mod prim;

use crate::ops::{fetch_distances, get_feeder_counts, get_unique_ids, insert_edges_into_lines};
use crate::prim::prims_algorithm;

fn main() {
    let mut connection = establish_connection();
    let feeder_counts = get_feeder_counts(&mut connection);
    match feeder_counts {
        Ok(feeder_counts) => {
            for feeder_count in feeder_counts {
                if let Some(feeder_id) = feeder_count.feeder_id {
                    println!("Feeder ID: {}, Count: {}", feeder_id, feeder_count.count);

                    // Fetch distances for each feeder_id
                    match fetch_distances(&mut connection, feeder_id) {
                        Ok(distances) => {
                            let ids = get_unique_ids(&mut connection, feeder_id).unwrap();
                            let result = prims_algorithm(&distances, &ids);
                            insert_edges_into_lines(&mut connection, result, feeder_id).expect("Insertion failed miserably");
                        },
                        Err(e) => eprintln!("Database error in fetch_distances: {}", e),
                    }
                } else {
                    println!("Feeder ID is None, Count: {}", feeder_count.count);
                }
            }
        },
        Err(e) => eprintln!("Database error in get_feeder_counts: {}", e),
    }
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok(); // Load the .env file

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}
