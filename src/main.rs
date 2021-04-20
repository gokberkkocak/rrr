mod app;
mod csv_dump;
mod db;
mod json;
mod plot_helper;
mod utils;

use json::{ExperimentSingle, ExperimentStore};
use tokio::runtime::Runtime;
use utils::Mode;

fn main() {
    let matches = app::RRRApp::get_matches();
    if let Some(json_matches) = matches.subcommand_matches("local") {
        let file_name = json_matches.value_of("input").unwrap();
        // different behaviour depending on file is compressed or folder_dump
        let decompress = json_matches.is_present("decompress");
        let folder = json_matches.is_present("folder");
        let mut store = match folder {
            true => ExperimentStore::from_folder(file_name, decompress),
            false => {
                let data = utils::read_file(file_name, decompress);
                serde_json::from_str(data.as_str()).unwrap()
            }
        };
        if let Some(sub_matches) = json_matches.subcommand_matches("time") {
            let experiment_id = sub_matches.value_of("experiment_id").unwrap();
            let config_id = sub_matches.value_of("config_id").unwrap();
            let mode = if sub_matches.is_present("sr_time") {
                Mode::SRTime
            } else {
                Mode::SolverTime
            };
            json::check_mode(&store, experiment_id, config_id, mode, true);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("best-time") {
            let experiment_id = sub_matches.value_of("experiment_id").unwrap();
            let mode = if sub_matches.is_present("sr_time") {
                Mode::SRTime
            } else {
                Mode::SolverTime
            };
            json::check_mode(&store, experiment_id, "", mode, false);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("sol") {
            let experiment_id = sub_matches.value_of("experiment_id").unwrap();
            json::check_mode(&store, experiment_id, "", Mode::NbSolutions, false);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("write") {
            let pretty = sub_matches.is_present("pretty");
            let maintenance = sub_matches.is_present("maintenance");
            if let Some(t) = sub_matches.values_of("add") {
                let input_files: Vec<&str> = t.collect();
                json::merge_mode(&mut store, input_files);
            }
            if maintenance {
                json::fix_doubts(&mut store);
            }
            let new_json = if pretty {
                serde_json::to_string_pretty(&store).unwrap()
            } else {
                serde_json::to_string(&store).unwrap()
            };
            let output_file = sub_matches.value_of("output").unwrap();
            // Compressed the file if specified.
            let compress = sub_matches.is_present("compress");
            utils::write_to_file(output_file, new_json, compress);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("convert") {
            let pretty = sub_matches.is_present("pretty");
            let output_file = sub_matches.value_of("output").unwrap();
            let plot_store = plot_helper::convert_store_for_plot(&store);
            let new_json = if pretty {
                serde_json::to_string_pretty(&plot_store).unwrap()
            } else {
                serde_json::to_string(&plot_store).unwrap()
            };
            utils::write_to_file(output_file, new_json, false);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("csv-dump") {
            let output_file = sub_matches.value_of("output").unwrap();
            let table = sub_matches.is_present("table");
            let data = if table {
                store.convert_to_csv_table()
            } else {
                store.convert_to_csv()
            };
            utils::write_to_file(output_file, data, false);
        } else if let Some(sub_matches) = json_matches.subcommand_matches("folder-dump") {
            let output_folder = sub_matches.value_of("output").unwrap();
            let compress = sub_matches.is_present("compress");
            store.folder_dump(output_folder, compress);
        }
    } else if let Some(db_matches) = matches.subcommand_matches("remote") {
        let rt = Runtime::new().unwrap();
        rt.block_on(async {
            let db_config_file = db_matches.value_of("db").unwrap();
            let data = utils::read_file(db_config_file, false);
            let db_config = db::DBConfig::from_config_file(&data);
            let mut conn = db::init_db_connection(db_config).await;
            if let Some(sub_matches) = db_matches.subcommand_matches("init") {
                db::drop_table(&mut conn).await;
                db::create_table(&mut conn).await;
                if sub_matches.is_present("input") {
                    let file_name = sub_matches.value_of("input").unwrap();
                    let data = utils::read_file(file_name, false);
                    let store: ExperimentStore = serde_json::from_str(data.as_str()).unwrap();
                    println!("Depending on the number of experiments, this might take a while.");
                    db::populate_db(&mut conn, store).await;
                }
            } else if let Some(sub_matches) = db_matches.subcommand_matches("sol") {
                let experiment_id = sub_matches.value_of("experiment_id").unwrap();
                db::check_mode(&mut conn, experiment_id, "", Mode::NbSolutions, false).await;
            } else if let Some(sub_matches) = db_matches.subcommand_matches("time") {
                let experiment_id = sub_matches.value_of("experiment_id").unwrap();
                let config_id = sub_matches.value_of("config_id").unwrap();
                db::check_mode(&mut conn, experiment_id, config_id, Mode::SRTime, true).await;
            } else if let Some(sub_matches) = db_matches.subcommand_matches("best-time") {
                let experiment_id = sub_matches.value_of("experiment_id").unwrap();
                db::check_mode(&mut conn, experiment_id, "", Mode::SRTime, false).await;
            } else if let Some(sub_matches) = db_matches.subcommand_matches("commit") {
                let exp_file = sub_matches.value_of("add").unwrap();
                let data = utils::read_file(exp_file, false);
                let experiment: ExperimentSingle = serde_json::from_str(data.as_str()).unwrap();
                db::commit_to_db(&mut conn, experiment).await;
            } else if let Some(sub_matches) = db_matches.subcommand_matches("nb-success") {
                let experiment_id = sub_matches.value_of("experiment_id").unwrap();
                let config_id = sub_matches.value_of("config_id").unwrap();
                let nb = db::check_nb_successful(&mut conn, experiment_id, config_id).await;
                println!("DB_NB_SUCCESS_DIFF_SEED {}", nb);
            }
        });
    }
}
