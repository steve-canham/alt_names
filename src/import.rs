use sqlx::{Pool, Postgres};
use crate::AppError;
use std::path::PathBuf;

pub async fn import_data(_data_folder: &PathBuf, _source_file_name: &PathBuf, _pool: &Pool<Postgres>) -> Result<(), AppError> {

    Ok(())
}

pub async fn summarise_import(_pool: &Pool<Postgres>) -> Result<(), AppError> {

    Ok(())
}

        