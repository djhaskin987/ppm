mod init;
mod install;
use install::install_package;
use init::initialize_package_database;
extern crate clap;
use clap::{Arg, App, SubCommand};
use path_clean::PathClean;
use std::path::PathBuf;


fn main() -> Result<(),Box<dyn std::error::Error>> {
    let matches = App::new("Project Package Manager")
        .version("0.1.0")
        .author("Daniel Jay Haskin")
        .about("Package Manager for Projects")
        .arg(Arg::with_name("config")
             .short("c")
             .long("config")
             .value_name("FILE")
             .help("Use a custom config file")
             .takes_value(true))
        .arg(Arg::with_name("root_path")
             .short("p")
             .long("root-path")
             .value_name("PATH")
             .help("Use a custom root path")
             .takes_value(true))
       .arg(Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .multiple(true)
             .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("init")
                    .about("Initialize package database"))
        .subcommand(SubCommand::with_name("install")
                    .about("Install (or upgrade) one or more packages")
                    .arg(Arg::with_name("LOCATION")
                         .required(true)
                         .help("Location of the package to install")))
        .subcommand(SubCommand::with_name("remove")
                    .about("Remove one or more packages")
                    .arg(Arg::with_name("NAME")
                         .help("Package name to remove")
                         .required(true)))
        .get_matches();

    // Use PWD if root path was not specified
    let root_path = matches
        .value_of("root_path")
        .map_or(
            std::env::current_dir().unwrap_or(PathBuf::from(".")),
            |p| { PathBuf::from(p) }
        );
    let config_path = matches
        .value_of("config").map_or(
            root_path.as_path().join("ppm.yml"),
            |f| { root_path.as_path().join(f).clean() }
        );
    println!("{:?}", matches);
    println!("{:?}", root_path);
    println!("{:?}", config_path);
    let verbosity_level = matches.occurrences_of("v");
    println!("{:?}", verbosity_level);

    if let Some(_) = matches.subcommand_matches("init") {
        initialize_package_database(&root_path)?;
    } else if let Some(matches) = matches.subcommand_matches("install") {
        let location = matches.value_of("LOCATION").unwrap();
        install_package(location)?;
    }

    return Ok(());
}
