use diesel::PgConnection;
use uuid::Uuid;

use super::models::InsertLocation;

pub fn generate_speculative(id: Uuid, insert_data: InsertLocation, con: &mut PgConnection) {}
