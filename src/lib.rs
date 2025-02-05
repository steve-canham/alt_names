
pub mod setup;
pub mod error_defs;
mod initialise;
mod import;
mod export;

use error_defs::AppError;
use setup::log_helper;
use std::ffi::OsString;

pub async fn run(args: Vec<OsString>) -> Result<(), AppError> {

    let params = setup::get_params(args).await?;
    let flags = params.flags;
    let test_run = flags.test_run;

    if !test_run {
       log_helper::setup_log(&params.log_folder, &params.source_file_name)?;
       log_helper::log_startup_params(&params);
    }
            
    let pool = setup::get_db_pool().await?;

    // Processing of the remaining stages depends on the 
    // presence of the relevant CLI flag(s).
    
    if flags.initialise {
           
           // do any initialisation required - e.g. create tables
           initialise::create_geo_tables(&pool).await?;

    }
    else  {
        if flags.import_data   // import ror from json file and store in ror schema tables
        {
            import::import_data(&params.data_folder, &params.source_file_name, &pool).await?;
            if !test_run {
                import::summarise_import(&pool).await?;
            }
        }

        if flags.export_data  // write out summary data from data in smm tables
        { 
            export::export_data(&params.output_folder, &params.source_file_name, &pool).await?;
        }
    }

     Ok(())  
}
