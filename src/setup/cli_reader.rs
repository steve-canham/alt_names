/***************************************************************************
 * Module uses clap crate to read command line arguments. These include 
 * possible A, S, T and C flags, and possible strings for the data folder and 
 * source file name. If no flags 'S' (= import data) is returned by default.
 * Folder and file names return an empty string ("") rather than null if not 
 * present. 
 ***************************************************************************/

use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;
use std::path::PathBuf;

pub struct CliPars {
    pub source_file: PathBuf,
    pub flags: Flags, 
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub import_data: bool,
    pub export_data: bool,
    pub initialise: bool,
    pub test_run: bool,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args)?;

    // These parameters guaranteed to unwrap OK as all have a default value of "".

    let source_file_as_string = parse_result.get_one::<String>("src_file").unwrap();
    let source_file = PathBuf::from(source_file_as_string);

    // Flag values are false if not present, true if present.

    let i_flag = parse_result.get_flag("i_flag");
    let mut r_flag = parse_result.get_flag("r_flag");
    let mut x_flag = parse_result.get_flag("x_flag");
    let z_flag = parse_result.get_flag("z_flag");

    // If c, m, or both flags set (may be by using 'i' (initialise) flag)
    // Only do the c and / or m actions
  
    if i_flag {

        let flags = Flags {
            import_data: false,
            export_data: false,
            initialise: i_flag,
            test_run: false,
        };

        Ok(CliPars {
            source_file: PathBuf::new(),
            flags: flags,
        })
    }
    
    else {
        if r_flag && x_flag {  
           x_flag = false;  // import is the default
        }

        if !r_flag && !x_flag {
            r_flag = true;  // import is the default
        }
    
        let flags = Flags {
            import_data: r_flag,
            export_data: x_flag,
            initialise: false,
            test_run: z_flag,
        };

        Ok(CliPars {
            source_file: source_file.clone(),
            flags: flags,
        })
    }
}


fn parse_args(args: Vec<OsString>) -> Result<ArgMatches, clap::Error> {

    command!()
        .about("Imports data from txt file and imports it into a database")
        .arg(
             Arg::new("src_file")
            .short('s')
            .long("source")
            .visible_aliases(["source file"])
            .help("A string with the source file name (over-rides environment setting")
            .default_value("")
        )
        .arg(
            Arg::new("r_flag")
           .short('r')
           .long("import")
           .required(false)
           .help("A flag signifying import from ror file to ror schema tables only")
           .action(clap::ArgAction::SetTrue)
        )
       .arg(
             Arg::new("x_flag")
            .short('x')
            .long("filesout")
            .required(false)
            .help("A flag signifying output a summary of the current or specified version into csv files")
            .action(clap::ArgAction::SetTrue)
        )
       .arg(
            Arg::new("i_flag")
           .short('i')
           .long("install")
           .required(false)
           .help("A flag signifying initial run, creates summary and context tables only")
           .action(clap::ArgAction::SetTrue)
       )
       .arg(
            Arg::new("z_flag")
            .short('z')
            .long("test")
            .required(false)
            .help("A flag signifying that this is part of an integration test run - suppresses logs")
            .action(clap::ArgAction::SetTrue)
       )
    .try_get_matches_from(args)

}


#[cfg(test)]
mod tests {
    use super::*;
    
    // Ensure the parameters are being correctly extracted from the CLI arguments

    #[test]
    fn check_cli_no_explicit_params() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();
        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::new());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, false);
    }
  
    #[test]
    fn check_cli_with_x_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-x"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::new());
        assert_eq!(res.flags.import_data, false);
        assert_eq!(res.flags.export_data, true);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_r_and_x_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-r", "-x"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::new());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_i_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-i"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::new());
        assert_eq!(res.flags.import_data, false);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, true);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_z_flags() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-z"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::new());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, true);
    }

    
    #[test]
    fn check_cli_with_explicit_string_pars() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "schema2 data.json"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::from("schema2 data.json"));
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_most_params_explicit() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "schema2.1 data.json", "-r", "-x", "-z"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, PathBuf::from("schema2.1 data.json"));
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.initialise, false);
        assert_eq!(res.flags.test_run, true);
    }

}

