use clap::{App, Arg, ArgGroup, ArgMatches};

pub struct RRRApp;

impl RRRApp {
    pub fn get_matches() -> ArgMatches {
        App::new("rrr")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Result Reader Rust")
        .subcommand(App::new("local")
            .about("Use RRR in local JSON file(s) mode")
            .arg(
                Arg::new("input")
                    .short('i')
                    .long("input")
                    .help("Sets the json file to use")
                    .takes_value(true)
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
            .subcommand(App::new("time")
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
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this.")
                        .takes_value(true)
                        .required(true),
                )
                .group(
                    ArgGroup::new("times")
                        .args(&["sr_time", "solver_time"])
                        .required(true),
                )
            )
            .subcommand(App::new("write")
                .about("Writes to json, merges the side input into main and deletes sides.")
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .value_name("SIDE_INPUT")
                        .help("Files to be consumed and merged")
                        .takes_value(true)
                        .multiple_occurrences(true)
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
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(App::new("convert")
                .about("Converts json to the plotter suited version.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("pretty")
                        .short('p')
                        .long("pretty")
                        .help("Pretty prints the output json")
                )
            )
            .subcommand(App::new("csv-dump")
                .about("Converts json as csv for R.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("File to write on")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("table")
                        .short('t')
                        .long("table")
                        .help("Dumps table format CSV")
                )
            )
            .subcommand(App::new("folder-dump")
                .about("Converts json into multiple jsons in a folder.")
                .arg(
                    Arg::new("output")
                        .short('o')
                        .long("output")
                        .value_name("OUTPUT")
                        .help("Folder to write")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("compress")
                        .short('c')
                        .long("compress")
                        .help("Set if you want to compress with zstd")
                )
            )
            .subcommand(App::new("sol")
                .about("Brings the number of solution of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(App::new("best-time")
                .about("Brings the best time of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
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
        .subcommand(App::new("remote")
            .about("Use RRR in remote MySQL DB mode")
            .arg(
                Arg::new("db")
                    .short('d')
                    .long("db-config")
                    .value_name("DB_CONFIG")
                    .help("DB conf file")
                    .takes_value(true)
                    .required(true),
            )
            .subcommand(App::new("init")
                .about("Init/clear the table and optionally populate from json")
                .arg(
                    Arg::new("input")
                        .short('i')
                        .long("input")
                        .value_name("MAIN_JSON")
                        .help("Main storage file json file to populate the db")
                        .takes_value(true)
                )
            )
            .subcommand(App::new("sol")
                .about("Brings the number of solutions of an instance")
                .arg(
                Arg::new("experiment_id")
                    .short('e')
                    .long("experiment")
                    .value_name("EXPERIMENT")
                    .help("Give experiment id which is model_instance_freq")
                    .takes_value(true)
                    .required(true),
                )
            )
            .subcommand(App::new("best-time")
                .about("Brings the best sr time of an instance")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(App::new("time")
                .about("Finds the exact min time of an instance from the db")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(App::new("nb-success")
                .about("Checks the db to find how many distinct seed successful runs on db.")
                .arg(
                    Arg::new("experiment_id")
                        .short('e')
                        .long("experiment")
                        .value_name("EXPERIMENT")
                        .help("Give experiment id which is model_instance_freq")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::new("config_id")
                        .short('c')
                        .long("config")
                        .value_name("CONFIG")
                        .help("Give config id which consists of many fields. miner_lite.py knows how to produce this")
                        .takes_value(true)
                        .required(true),
                )
            )
            .subcommand(App::new("commit")
                .about("Commits the new entry to db")
                .arg(
                    Arg::new("add")
                        .short('a')
                        .long("add")
                        .value_name("EXPERIMENT")
                        .help("Give single experiment in json format.")
                        .takes_value(true)
                        .required(true),
                )
            )
        )
        .get_matches()
    }
}
