use clap::{Arg, ArgGroup, ArgMatches, ArgAction, Command};

pub struct RRRApp;

impl RRRApp {
    pub fn get_matches() -> ArgMatches {
        Command::new("rrr")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Result Reader Rust")
        .subcommand(Command::new("local")
            .about("Use RRR in local JSON file(s) mode")
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("Sets the json file to use")
                    .action(ArgAction::SetTrue)
                    .required(true),
            )
            .arg(
                Arg::new("decompress")
                    .short('d')
                    .long("decompress")
                    .help("Set if JSON file is compressed with zstd")
            )
            .arg(
                Arg::new("folder")
                    .short('f')
                    .long("folder")
                    .help("Set if you want to give folder dump rather than single JSON file")
            )
            .subcommand(Command::new("time")
                .about("Brings the exact min time of an instance")
                .arg(
                    Arg::new("sr_time")
                        .short('r')
                        .long("savilerow-time")
                        .help("Use SR time"),
                )
                .arg(
                    Arg::new("solver_time")
                        .short('s')
                        .long("solver-time")
                        .help("Use solver time"),
                )
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this.")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .group(
                    ArgGroup::new("times")
                        .args(&["sr_time", "solver_time"])
                        .required(true),
                )
            )
            .subcommand(Command::new("write")
                .about("Writes to json, merges the side input into main and deletes sides.")
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .value_name("SIDE_INPUT")
                        .help("Files to be consumed and merged")
                        .action(ArgAction::SetTrue)
                        .action(ArgAction::Append)
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty prints the output json")
                )
                .arg(
                    Arg::new("maintenance")
                        .short('m')
                        .long("maintenance")
                        .help("Apply maintenance to json for doubted values")
                )
                .arg(
                    Arg::new("compress")
                        .short('c')
                        .long("compress")
                        .help("Set if you want to compress with zstd")
                )
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
            .subcommand(Command::new("convert")
                .about("Converts json to the plotter suited version.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty prints the output json")
                )
            )
            .subcommand(Command::new("csv-dump")
                .about("Converts json as csv for R.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("table")
                        .short('t')
                        .long("table")
                        .help("Dumps table format CSV")
                )
            )
            .subcommand(Command::new("folder-dump")
                .about("Converts json into multiple jsons in a folder.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Folder to write")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("compress")
                        .short('c')
                        .long("compress")
                        .help("Set if you want to compress with zstd")
                )
            )
            .subcommand(Command::new("sol")
                .about("Brings the number of solution of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
            .subcommand(Command::new("best-time")
                .about("Brings the best time of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("sr_time")
                        .short('r')
                        .long("savilerow-time")
                        .help("Use SR time"),
                )
                .arg(
                    Arg::new("solver_time")
                        .short('s')
                        .long("solver-time")
                        .help("Use solver time"),
                )
                .group(
                    ArgGroup::new("times")
                        .args(&["sr_time", "solver_time"])
                        .required(true),
                )
            )
        )
        // DB     
        .subcommand(Command::new("remote")
            .about("Use RRR in remote MySQL DB mode")
            .arg(
                Arg::new("db")
                    .short('d')
                    .long("db-config")
                    .value_name("DB_CONFIG")
                    .help("DB conf file")
                    .action(ArgAction::SetTrue)
                    .required(true),
            )
            .subcommand(Command::new("init")
                .about("Init/clear the table and optionally populate from json")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("MAIN_JSON")
                        .help("Main storage file json file to populate the db")
                        .action(ArgAction::SetTrue)
                )
            )
            .subcommand(Command::new("sol")
                .about("Brings the number of solutions of an instance")
                .arg(
                Arg::new("experiment_id")
                    .short('e')
                    .long("experiment")
                    .value_name("EXPERIMENT")
                    .help("Give experiment id which is model_instance_freq")
                    .action(ArgAction::SetTrue)
                    .required(true),
                )
            )
            .subcommand(Command::new("best-time")
                .about("Brings the best sr time of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
            .subcommand(Command::new("time")
                .about("Finds the exact min time of an instance from the db")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
            .subcommand(Command::new("nb-success")
                .about("Checks the db to find how many distinct seed successful runs on db.")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
            .subcommand(Command::new("commit")
                .about("Commits the new entry to db")
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .value_name("EXPERIMENT")
                        .help("Give single experiment in json format.")
                        .action(ArgAction::SetTrue)
                        .required(true),
                )
            )
        )
        .get_matches()
    }
}
