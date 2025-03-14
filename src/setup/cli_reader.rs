/***************************************************************************
 *
 ***************************************************************************/

use clap::{command, Arg, ArgMatches};
use crate::err::AppError;
use std::ffi::OsString;

pub struct CliPars {
    pub source_file: String,
    pub flags: Flags, 
}

#[derive(Debug, Clone, Copy)]
pub struct Flags {
    pub import_data: bool,
    pub include_nonlatin: bool,
    pub export_data: bool,
    pub test_run: bool,
}

pub fn fetch_valid_arguments(args: Vec<OsString>) -> Result<CliPars, AppError>
{ 
    let parse_result = parse_args(args)?;
    let source_file = parse_result.get_one::<String>("src_file").unwrap();
  
    // Flag values are false if not present, true if present.

    let mut r_flag = parse_result.get_flag("r_flag");
    let n_flag = parse_result.get_flag("n_flag");
    let mut x_flag = parse_result.get_flag("x_flag");
    let z_flag = parse_result.get_flag("z_flag");
     
    if r_flag && x_flag {  
        x_flag = false;  // import is the default
    }

    if !r_flag && !x_flag {
        r_flag = true;  // import is the default
    }

    let flags = Flags {
        import_data: r_flag,
        include_nonlatin: n_flag,
        export_data: x_flag,
        test_run: z_flag,
    };

    Ok(CliPars {
        source_file: source_file.clone(),
        flags: flags,
    })

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
            Arg::new("n_flag")
           .short('n')
           .long("non_latin")
           .required(false)
           .help("A flag signifying that non Latin names shopuld be included (are excludede by dfefault)")
           .action(clap::ArgAction::SetTrue)
        )
       .arg(
             Arg::new("x_flag")
            .short('x')
            .long("filesout")
            .required(false)
            .help("A flag signifying output a summary of the current geonames data into a csv file")
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
        assert_eq!(res.source_file, "".to_string());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, false);
    }
  
    #[test]
    fn check_cli_with_x_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-x"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "".to_string());
        assert_eq!(res.flags.import_data, false);
        assert_eq!(res.flags.export_data, true);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_r_and_x_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-r", "-x"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "".to_string());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.include_nonlatin, false);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_n_flag() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-n"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "".to_string());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.include_nonlatin, true);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_z_flags() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-z"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "".to_string());
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.include_nonlatin, false);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, true);
    }

    
    #[test]
    fn check_cli_with_explicit_string_pars() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "schema2 data.json"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "schema2 data.json");
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.include_nonlatin, false);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, false);
    }

    #[test]
    fn check_cli_with_most_params_explicit() {
        let target = "dummy target";
        let args : Vec<&str> = vec![target, "-s", "schema2.1 data.json", "-r", "-x", "-z"];
        let test_args = args.iter().map(|x| x.to_string().into()).collect::<Vec<OsString>>();

        let res = fetch_valid_arguments(test_args).unwrap();
        assert_eq!(res.source_file, "schema2.1 data.json");
        assert_eq!(res.flags.import_data, true);
        assert_eq!(res.flags.include_nonlatin, false);
        assert_eq!(res.flags.export_data, false);
        assert_eq!(res.flags.test_run, true);
    }

}

