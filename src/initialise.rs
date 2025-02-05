use sqlx::{Pool, Postgres};
use crate::AppError;

pub async fn create_geo_tables(_pool: &Pool<Postgres>) -> Result<(), AppError> {

    Ok(())

}