use diesel::prelude::*;
use crate::model::Category;
use crate::schema::expense_tags::dsl::*;
use diesel::r2d2::{Pool, ConnectionManager};
use diesel::PgConnection;

pub fn load_categories(
    pool: &Pool<ConnectionManager<PgConnection>>
) -> Result<Vec<Category>, String> {
    let mut conn = pool.get().map_err(|e| format!("Pool error: {:?}", e))?;
    expense_tags
        .load::<Category>(&mut conn)
        .map_err(|e| format!("Query error: {:?}", e))
}