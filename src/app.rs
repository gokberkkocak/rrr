use clap::{App, Arg, ArgGroup, ArgMatches, SubCommand};

pub struct RRRApp;

impl RRRApp {
    pub fn get_matches<'a>() -> ArgMatches<'a> {
        let matches = App::new("rrr")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Result Reader Rust")
        .subcommand(SubCommand::with_name("local")
            .about("Use RRR in local JSON file(s) mode")
            .arg(
                Arg::with_name("input")
                    .short("i")
                    .long("input")
                    .help("Sets the json file to use")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("decompress")
                    .short("d")
                    .long("decompress")
                    .help("Set if JSON file is compressed with zstd")
            )
            .arg(
                Arg::with_name("folder")
                    .short("f")
                    .long("folder")
                    .help("Set if you want to give folder dump rather than single JSON file")
            )
            .subcommand(SubCommand::with_name("time")
                .about("Brings the exact min time of an instance")
                .arg(
                    Arg::with_name("sr_time")
                        .short("r")
                        .long("savilerow-time")
                        .help("Use SR time"),
                )
                .arg(
                    Arg::with_name("solver_time")
                        .short("s")
                        .long("solver-time")
                        .help("Use solver time"),
                )
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("config_id")
                        .short("c")
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this.")
                        .takes_value(true)
                        .required(true),
                )
                .group(
                    ArgGroup::with_name("times")
                        .args(&["sr_time", "solver_time"])
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("write")
                .about("Writes to json, merges the side input into main and deletes sides.")
                .arg(
                    Arg::with_name("add")
                        .short("a")
                        .long("add")
                        .value_name("SIDE_INPUT")
                        .help("Files to be consumed and merged")
                        .takes_value(true)
                        .multiple(true)
                )
                .arg(
                    Arg::with_name("pretty")
                        .short("p")
                        .long("pretty")
                        .help("Pretty prints the output json")
                )
                .arg(
                    Arg::with_name("maintenance")
                        .short("m")
                        .long("maintenance")
                        .help("Apply maintenance to json for doubted values")
                )
                .arg(
                    Arg::with_name("compress")
                        .short("c")
                        .long("compress")
                        .help("Set if you want to compress with zstd")
                )
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("convert")
                .about("Converts json to the plotter suited version.")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("pretty")
                        .short("p")
                        .long("pretty")
                        .help("Pretty prints the output json")
                )
            )
            .subcommand(SubCommand::with_name("csv-dump")
                .about("Converts json as csv for R.")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("table")
                        .short("t")
                        .long("table")
                        .help("Dumps table format CSV")
                )
            )
            .subcommand(SubCommand::with_name("folder-dump")
                .about("Converts json into multiple jsons in a folder.")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Folder to write")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("compress")
                        .short("c")
                        .long("compress")
                        .help("Set if you want to compress with zstd")
                )
            )
            .subcommand(SubCommand::with_name("sol")
                .about("Brings the number of solution of an instance")
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("best-time")
                .about("Brings the best time of an instance")
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("sr_time")
                        .short("r")
                        .long("savilerow-time")
                        .help("Use SR time"),
                )
                .arg(
                    Arg::with_name("solver_time")
                        .short("s")
                        .long("solver-time")
                        .help("Use solver time"),
                )
                .group(
                    ArgGroup::with_name("times")
                        .args(&["sr_time", "solver_time"])
                        .required(true),
                )
            )
        )
        // DB     
        .subcommand(SubCommand::with_name("remote")
            .about("Use RRR in remote MySQL DB mode")
            .arg(
                Arg::with_name("db")
                    .short("d")
                    .long("db-config")
                    .value_name("DB_CONFIG")
                    .help("DB conf file")
                    .takes_value(true)
                    .required(true),
            )
            .subcommand(SubCommand::with_name("init")
                .about("Init/clear the table and optionally populate from json")
                .arg(
                    Arg::with_name("input")
                        .short("i")
                        .long("input")
                        .value_name("MAIN_JSON")
                        .help("Main storage file json file to populate the db")
                        .takes_value(true)
                )
            )
            .subcommand(SubCommand::with_name("sol")
                .about("Brings the number of solutions of an instance")
                .arg(
                Arg::with_name("experiment_id")
                    .short("e")
                    .long("experiment")
                    .value_name("EXPERIMENT")
                    .help("Give experiment id which is model_instance_freq")
                    .takes_value(true)
                    .required(true),
                )
            )
            .subcommand(SubCommand::with_name("best-time")
                .about("Brings the best sr time of an instance")
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("time")
                .about("Finds the exact min time of an instance from the db")
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("config_id")
                        .short("c")
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("nb-success")
                .about("Checks the db to find how many distinct seed successful runs on db.")
                .arg(
                    Arg::with_name("experiment_id")
                        .short("e")
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("config_id")
                        .short("c")
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(SubCommand::with_name("commit")
                .about("Commits the new entry to db")
                .arg(
                    Arg::with_name("add")
                        .short("a")
                        .long("add")
                        .value_name("EXPERIMENT")
                        .help("Give single experiment in json format.")
                        .takes_value(true)
                        .required(true),
                )
            )
        )
        .get_matches();
        matches
    }
}
