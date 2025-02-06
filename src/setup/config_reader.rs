/***************************************************************************
 * Module uses 
 * ...
 * ...
 * 
 * Database parameters MUST be provided and be valid or the program can not
 * continue. 
 * 
 * 
 ***************************************************************************/

use std::sync::OnceLock;
use toml;
use serde::Deserialize;
use crate::error_defs::{AppError, CustomError};
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct TomlConfig {
    pub files: Option<TomlFilePars>, 
    pub database: Option<TomlDBPars>,
}

#[derive(Debug, Deserialize)]
pub struct TomlFilePars {
    pub data_folder_path: Option<String>,
    pub log_folder_path: Option<String>,
    pub output_folder_path: Option<String>,
    pub src_file_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TomlDBPars {
    pub db_host: Option<String>,
    pub db_user: Option<String>,
    pub db_password: Option<String>,
    pub db_port: Option<String>,
    pub db_name: Option<String>,
}

pub struct Config {
    pub files: FilePars, 
    pub db_pars: DBPars,
}

pub struct FilePars {
    pub data_folder_path: PathBuf,
    pub log_folder_path: PathBuf,
    pub output_folder_path: PathBuf,
    pub src_file_name: PathBuf,
}

#[derive(Debug, Clone)]
pub struct DBPars {
    pub db_host: String,
    pub db_user: String,
    pub db_password: String,
    pub db_port: usize,
    pub db_name: String,
}

pub static DB_PARS: OnceLock<DBPars> = OnceLock::new();

pub fn populate_config_vars(config_string: &String) -> Result<Config, AppError> {
    
    let toml_config = match toml::from_str::<TomlConfig>(&config_string)
    {
        Ok(c) => c,
        Err(_) => { 
            let app_err = report_critical_error ("open config file", 
            "the correct name, form and is in the correct location");
            return Result::Err(app_err)   // return error to calling function
        },
    };


    let toml_database = match toml_config.database {
        Some(d) => d,
        None => {
            let app_err = report_critical_error ("read DB parameters from config file", 
            "a set of values under table 'database'");
            return Result::Err(app_err)
        },
    };


    let toml_files = match toml_config.files {
        Some(f) => f,
        None => {
            let app_err = report_critical_error ("read file parameters from config file", 
            "a set of values under table 'files'");
            return Result::Err(app_err)
        },
    };
   
    let config_files = verify_file_parameters(toml_files)?;
    let config_db_pars = verify_db_parameters(toml_database)?;

    let _ = DB_PARS.set(config_db_pars.clone());

    Ok(Config{
        files: config_files,
        db_pars: config_db_pars,
    })
}


fn report_critical_error (error_suffix: &str, sec2: &str) -> AppError {
 
    let print_msg = r#"CRITICAL ERROR - Unable to "#.to_string() + error_suffix + r#" - 
    program cannot continue. Please check config file ('config_imp_ror.toml') 
    has "# + sec2;
    println!("{}", print_msg);

    let err_msg = format!("CRITICAL ERROR - Unable to {}", error_suffix);
    let cf_err = CustomError::new(&err_msg);

    AppError::CsErr(cf_err) 
}


fn verify_file_parameters(toml_files: TomlFilePars) -> Result<FilePars, AppError> {

    // Check data folder and source file first as there are no defaults for these values.
    // They must therefore be present.

    let data_folder_path = check_critical_pathbuf (toml_files.data_folder_path, "read data folder path from config file", 
               "a value for data_folder_path")?;

    let src_file_name = check_critical_pathbuf (toml_files.src_file_name, "read source file from config file", 
                "a value for src_file_name")?;
               
    let log_folder_path = check_pathbuf (toml_files.log_folder_path, "log folder", &data_folder_path);

    let output_folder_path = check_pathbuf (toml_files.output_folder_path, "outputs folder", &data_folder_path);

    Ok(FilePars {
        data_folder_path,
        log_folder_path,
        output_folder_path,
        src_file_name,
    })
}


fn check_critical_pathbuf (src_name: Option<String>, sec2: &str, error_suffix: &str) -> Result<PathBuf, AppError> {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        let print_msg = r#"CRITICAL ERROR - Unable to "#.to_string() + error_suffix + r#" - 
        program cannot continue. Please check config file ('config_imp_ror.toml') 
        has "# + sec2;
        println!("{}", print_msg);
    
        let err_msg = format!("CRITICAL ERROR - Unable to {}", error_suffix);
        let cf_err = CustomError::new(&err_msg);
        Err(AppError::CsErr(cf_err))
    }
    else {
        Ok(PathBuf::from(s))
    }
}


fn check_pathbuf (src_name: Option<String>, folder_type: &str, alt_path: &PathBuf) -> PathBuf {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        let print_msg = r#"No value found for "#.to_string() + folder_type + r#" path in config file - 
            using the provided data folder instead."#;
        println!("{}", print_msg);
        alt_path.to_owned()
    }
    else {
        PathBuf::from(s)
    }
}


fn verify_db_parameters(toml_database: TomlDBPars) -> Result<DBPars, AppError> {

// Check user name and password first as there are no defaults for these values.
    // They must therefore be present.

    let db_user = check_critical_db_par (toml_database.db_user , "a value for db_user", "read user name from config file")?; 

    let db_password = check_critical_db_par (toml_database.db_password , "a value for db_password", "read user password from config file")?; 

    let db_host = check_db_par (toml_database.db_host, "DB host", "localhost");
            
    let db_port_as_string = check_db_par (toml_database.db_port, "DB port", "5432");
    let db_port: usize = db_port_as_string.parse().unwrap_or_else(|_| 5432);

    let db_name = check_db_par (toml_database.db_name, "DB name", "geo");

    Ok(DBPars {
        db_host,
        db_user,
        db_password,
        db_port,
        db_name,
    })
}

fn check_critical_db_par (src_name: Option<String>, sec2: &str, error_suffix: &str) -> Result<String, AppError> {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        let print_msg = r#"CRITICAL ERROR - Unable to "#.to_string() + error_suffix + r#" - 
        program cannot continue. Please check config file ('config_imp_ror.toml') 
        has "# + sec2;
        println!("{}", print_msg);
    
        let err_msg = format!("CRITICAL ERROR - Unable to {}", error_suffix);
        let cf_err = CustomError::new(&err_msg);
        Err(AppError::CsErr(cf_err))
    }
    else {
        Ok(s)
    }
}


fn check_db_par (src_name: Option<String>, folder_type: &str, default:  &str) -> String {
 
    let s = match src_name {
        Some(s) => s,
        None => "none".to_string(),
    };

    if s == "none".to_string() || s.trim() == "".to_string()
    {
        let print_msg = r#"No value found for "#.to_string() + folder_type + r#" path in config file - 
            using the provided default value instead."#;
        println!("{}", print_msg);
        default.to_owned()
    }
    else {
       s
    }
}


pub fn fetch_db_name() -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            let msg = "Unable to obtain DB name when retrieving database name";
            let cf_err = CustomError::new(msg);
            return Result::Err(AppError::CsErr(cf_err));
        },
    };
    Ok(db_pars.db_name.clone())
}


pub fn fetch_db_conn_string(db_name: String) -> Result<String, AppError> {
    let db_pars = match DB_PARS.get() {
         Some(dbp) => dbp,
         None => {
            let msg = "Unable to obtain DB parameters when building connection string";
            let cf_err = CustomError::new(msg);
            return Result::Err(AppError::CsErr(cf_err));
        },
    };
    
    Ok(format!("postgres://{}:{}@{}:{}/{}", 
    db_pars.db_user, db_pars.db_password, db_pars.db_host, db_pars.db_port, db_name))
}



#[cfg(test)]
mod tests {
    use super::*;
    
    // Ensure the parameters are being correctly extracted from the config file string
    
    #[test]
    fn check_config_with_all_params_present() {

        let config = r#"

[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();
        assert_eq!(res.files.data_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.log_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\logs"));
        assert_eq!(res.files.output_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\outputs"));
        assert_eq!(res.files.src_file_name, PathBuf::from("alternateNamesV2.txt"));
        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5433);
        assert_eq!(res.db_pars.db_name, "geo");
    }


    #[test]
    fn check_config_with_missing_log_and_outputs_folders() {

        let config = r#"

[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();
        assert_eq!(res.files.data_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.log_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.output_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.src_file_name, PathBuf::from("alternateNamesV2.txt"));
    }


    #[test]
    fn check_config_with_blank_log_and_outputs_folders() {

        let config = r#"
[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path=""
output_folder_path=""
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();
        assert_eq!(res.files.data_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.log_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.output_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.src_file_name, PathBuf::from("alternateNamesV2.txt"));
    }


    #[test]
    fn check_missing_data_details_become_empty_strings() {

        let config = r#"
[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();
        assert_eq!(res.files.data_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\data"));
        assert_eq!(res.files.log_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\logs"));
        assert_eq!(res.files.output_folder_path, PathBuf::from("E:\\MDR source data\\Geonames\\outputs"));
        assert_eq!(res.files.src_file_name, PathBuf::from("alternateNamesV2.txt"));

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5433);
        assert_eq!(res.db_pars.db_name, "geo");
    }


    #[test]
    #[should_panic]
    fn check_missing_data_folder_panics() {
    let config = r#"
[files]
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="v1.59-2025-01-23-ror-data_schema_v2.json"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    #[should_panic]
    fn check_missing_user_name_panics() {

        let config = r#"
[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user=""
db_password="password"
db_port="5433"
db_name="geo"
"#;
        let config_string = config.to_string();
        let _res = populate_config_vars(&config_string).unwrap();
    }


    #[test]
    fn check_db_defaults_are_supplied() {

        let config = r#"
[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="alternateNamesV2.txt"

[database]
db_user="user_name"
db_password="password"
"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();
        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.db_name, "geo");
    }


#[test]
    fn missing_port_gets_default() {

        let config = r#"
[files]
data_folder_path="E:\\MDR source data\\Geonames\\data"
log_folder_path="E:\\MDR source data\\Geonames\\logs"
output_folder_path="E:\\MDR source data\\Geonames\\outputs"
src_file_name="alternateNamesV2.txt"

[database]
db_host="localhost"
db_user="user_name"
db_password="password"
db_port=""
db_name="geo"

"#;
        let config_string = config.to_string();
        let res = populate_config_vars(&config_string).unwrap();

        assert_eq!(res.db_pars.db_host, "localhost");
        assert_eq!(res.db_pars.db_user, "user_name");
        assert_eq!(res.db_pars.db_password, "password");
        assert_eq!(res.db_pars.db_port, 5432);
        assert_eq!(res.db_pars.db_name, "geo");
    }

}
  

