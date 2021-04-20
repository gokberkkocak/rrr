use serde::Serialize;

use csv::Writer;

use crate::json::{ExperimentStore, SolveInformation};
use crate::utils::Mean;

#[derive(Serialize)]
pub struct CSVView<'a> {
    experiment: &'a str,
    config: &'a str,
    solver_time: f64,
    total_time: f64,
}

impl<'a> CSVView<'a> {
    fn new(experiment: &'a str, config: &'a str, solver_time: f64, total_time: f64) -> Self {
        CSVView {
            experiment,
            config,
            solver_time,
            total_time,
        }
    }
}

impl ExperimentStore {
    pub fn convert_to_csv(&self) -> String {
        let mut writer = Writer::from_writer(vec![]);
        for (exp_key, exps) in &self.experiments {
            for (c_key, configs) in &exps.configs {
                let solves = &configs.solve_information;
                for i in solves {
                    if let SolveInformation::Success {
                        total_sr_time,
                        total_solver_time,
                        ..
                    } = i
                    {
                        let new_exp =
                            CSVView::new(exp_key, c_key, *total_solver_time, *total_sr_time);
                        writer.serialize(new_exp).unwrap();
                    }
                }
            }
        }
        String::from_utf8(writer.into_inner().expect("Cannot make it into"))
            .expect("Cannot convert to String")
    }

    pub fn convert_to_csv_table(&self) -> String {
        let mut writer = Writer::from_writer(vec![]);
        let mut config_names = vec![];
        // config_names.push(String::from("experiment_names"));
        // preprocess to get all config names
        for (_exp_key, exps) in &self.experiments {
            for (c_key, _configs) in &exps.configs {
                if !config_names.contains(c_key) {
                    config_names.push(c_key.to_string());
                }
            }
        }
        config_names.sort();
        config_names.insert(0, String::from("experiment_names"));
        writer
            .write_record(&config_names)
            .expect("couldn't write field names");
        // fill csv for each exp
        for (exp_key, exps) in &self.experiments {
            let mut line_vec = vec![String::from("NaN"); config_names.len()];
            line_vec[0] = exp_key.to_string();
            for (c_key, configs) in &exps.configs {
                let solves = &configs.solve_information;
                let mut sr_times = vec![];
                for i in solves {
                    if let SolveInformation::Success { total_sr_time, .. } = i {
                        sr_times.push(*total_sr_time);
                    }
                }
                let mean = sr_times.mean();
                let index = config_names
                    .iter()
                    .position(|x| x == c_key)
                    .expect("couldn't get column index");
                line_vec[index] = mean.to_string();
            }
            writer
                .write_record(line_vec)
                .expect("couldn't write csv file");
        }
        String::from_utf8(writer.into_inner().expect("Cannot make it into"))
            .expect("Cannot convert to String")
    }
}
