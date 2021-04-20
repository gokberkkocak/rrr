use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;

use crate::utils::{self, read_file, FSort, Mode};

#[derive(Serialize, Deserialize, Clone)]
pub struct LevelInformation {
    pub solver_time: Option<HashMap<String, f64>>,
    pub nodes: Option<HashMap<String, u64>>,
    pub cumulative_nb_solutions: Option<HashMap<String, u64>>,
    pub nb_vars: Option<HashMap<String, u64>>,
    pub nb_clauses: Option<HashMap<String, u64>>,
    pub nb_learnt_clauses: Option<HashMap<String, u64>>,
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum SolveInformation {
    #[serde(rename = "SUCCESS")]
    Success {
        total_solver_time: f64,
        total_sr_time: f64,
        total_nodes: Option<u64>,
        nb_solutions: u64,
        seed: Option<f64>,
        memory_limit: u64,
        time_limit: u64,
        machine_info: String,
        level_info: Box<LevelInformation>,
        // will remove this after dealing with new experiments.
        #[serde(default)]
        freq_nb_solutions: Option<u64>,
    },
    #[serde(rename = "DOUBTED")]
    Doubted {
        total_solver_time: f64,
        total_sr_time: f64,
        total_nodes: Option<u64>,
        nb_solutions: u64,
        seed: Option<f64>,
        memory_limit: u64,
        time_limit: u64,
        machine_info: String,
        level_info: Box<LevelInformation>,
        // will remove this after dealing with new experiments.
        #[serde(default)]
        freq_nb_solutions: Option<u64>,
    },
    #[serde(rename = "TIMEOUT")]
    Timeout {
        seed: Option<f64>,
        memory_limit: u64,
        time_limit: u64,
        machine_info: String,
    },
    #[serde(rename = "MEMOUT")]
    Memout {
        seed: Option<f64>,
        memory_limit: u64,
        time_limit: u64,
        machine_info: String,
        // will remove this after dealing with new experiments.
        #[serde(default)]
        crash_time: f64,
    },
    #[serde(rename = "CRASHED")]
    Crash {
        seed: Option<f64>,
        memory_limit: u64,
        time_limit: Option<u64>,
        machine_info: String,
        // will remove this after dealing with new experiments.
        #[serde(default)]
        crash_time: f64,
    },
}

#[derive(Serialize, Deserialize)]
pub struct ConfigMultiple {
    preprocess: String,
    representation: String,
    solver: String,
    incomparability: bool,
    interactive: bool,
    native: bool,
    compressed: bool,
    ordered: bool,
    no_solution_blocking: bool,
    mdd: bool,
    cgroups: bool,
    pub solve_information: Vec<SolveInformation>,
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentSingle {
    model: String,
    instance: String,
    freq: f64,
    pub exp_id: String,
    pub config_id: String,
    pub config: Box<ConfigMultiple>,
}
#[derive(Serialize, Deserialize)]
pub struct ExperimentMultiple {
    model: String,
    instance: String,
    freq: f64,
    pub configs: HashMap<String, Box<ConfigMultiple>>,
}

impl ExperimentMultiple {
    pub fn new(
        model: String,
        instance: String,
        freq: f64,
        configs: HashMap<String, Box<ConfigMultiple>>,
    ) -> ExperimentMultiple {
        ExperimentMultiple {
            model,
            instance,
            freq,
            configs,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ExperimentStore {
    pub experiments: HashMap<String, Box<ExperimentMultiple>>,
}

pub fn merge_mode(store: &mut ExperimentStore, input_files: Vec<&str>) {
    for f in input_files {
        let data = read_file(f, false);
        for line in data.lines() {
            let experiment: ExperimentSingle = serde_json::from_str(line).unwrap();
            merge_one_experiment(store, experiment);
        }
        fs::remove_file(f).expect("Unable to delete side file");
    }
}

fn merge_one_experiment(store: &mut ExperimentStore, experiment: ExperimentSingle) {
    // if exp id is in
    if let Some(exp_multi) = store.experiments.get_mut(experiment.exp_id.as_str()) {
        // if config id is in
        if let Some(config_multi) = exp_multi.configs.get_mut(experiment.config_id.as_str()) {
            assert_eq!(experiment.config.solve_information.len(), 1);
            let mut new_solve = experiment.config.solve_information;
            // check nb of solutions
            if let SolveInformation::Success {
                total_solver_time,
                total_sr_time,
                total_nodes,
                nb_solutions: new,
                seed,
                memory_limit,
                time_limit,
                machine_info,
                level_info,
                freq_nb_solutions,
            } = &new_solve[0]
            {
                for solve in config_multi.solve_information.iter() {
                    if let SolveInformation::Success {
                        nb_solutions: old, ..
                    } = solve
                    {
                        // if it is reporting different nb of sols, interpret as uncaught crash
                        if *old != *new {
                            let new_level_info = level_info.clone();
                            let second = SolveInformation::Doubted {
                                seed: *seed,
                                memory_limit: *memory_limit,
                                time_limit: *time_limit,
                                machine_info: machine_info.clone(),
                                total_sr_time: *total_sr_time,
                                total_solver_time: *total_solver_time,
                                total_nodes: *total_nodes,
                                nb_solutions: *new,
                                level_info: new_level_info,
                                freq_nb_solutions: *freq_nb_solutions,
                            };
                            new_solve.pop();
                            new_solve.push(second);
                            println!("An experiment gives different nb of solutions. Altering that experiment as DOUBTED: details; {} {}", experiment.exp_id, experiment.config_id);
                            break;
                        }
                    }
                }
            }
            // consumes vec by creating an iter
            config_multi
                .solve_information
                .push(new_solve.into_iter().next().unwrap());
        }
        // if config is not in
        else {
            exp_multi
                .configs
                .insert(experiment.config_id, experiment.config);
        }
    }
    // exp is not found
    else {
        let mut configs = HashMap::new();
        configs.insert(experiment.config_id, experiment.config);
        store.experiments.insert(
            experiment.exp_id,
            Box::new(ExperimentMultiple::new(
                experiment.model,
                experiment.instance,
                experiment.freq,
                configs,
            )),
        );
    }
}

pub fn check_mode(
    store: &ExperimentStore,
    experiment_id: &str,
    config_id: &str,
    mode: Mode,
    exact: bool,
) {
    // exp is in
    if let Some(experiment) = store.experiments.get(experiment_id) {
        if exact {
            // config is in
            if let Some(config) = experiment.configs.get(config_id) {
                match get_best_time(config) {
                    BestExperimentResult::Success {
                        min_total_sr_time,
                        min_total_solver_time,
                        nb_solutions,
                    } => match mode {
                        Mode::SRTime => println!("MIN_SR {}", min_total_sr_time),
                        Mode::SolverTime => println!("MIN_SOLVER {}", min_total_solver_time),
                        Mode::NbSolutions => println!("NB_SOLS {}", nb_solutions),
                    },
                    BestExperimentResult::Timeout { max_time_limit } => {
                        println!("MAX_TIMEOUT {}", max_time_limit);
                    }
                    BestExperimentResult::None => {
                        println!("EMPTY");
                    }
                }
            } else {
                println!("EMPTY");
            }
        } else {
            match get_virtual_best(&experiment.configs) {
                BestExperimentResult::Success {
                    min_total_sr_time,
                    min_total_solver_time,
                    nb_solutions,
                } => match mode {
                    Mode::SRTime => println!("MIN_SR {}", min_total_sr_time),
                    Mode::SolverTime => println!("MIN_SOLVER {}", min_total_solver_time),
                    Mode::NbSolutions => println!("NB_SOLS {}", nb_solutions),
                },
                _ => println!("EMPTY"),
            }
        }
    } else {
        println!("EMPTY");
    }
}

enum BestExperimentResult {
    Success {
        min_total_sr_time: f64,
        min_total_solver_time: f64,
        nb_solutions: u64,
    },
    Timeout {
        max_time_limit: u64,
    },
    None,
}

fn get_best_time(config: &ConfigMultiple) -> BestExperimentResult {
    let mut res_sr = vec![];
    let mut res_solver = vec![];
    let mut res_timeout = vec![];
    let mut res_nb_solutions: u64 = 0;
    for s in &config.solve_information {
        match s {
            SolveInformation::Success {
                total_sr_time,
                total_solver_time,
                nb_solutions,
                ..
            } => {
                res_sr.push(*total_sr_time);
                res_solver.push(*total_solver_time);
                if res_nb_solutions == 0 {
                    res_nb_solutions = *nb_solutions;
                }
            }
            SolveInformation::Timeout { time_limit, .. } => {
                res_timeout.push(*time_limit);
            }
            _ => (),
        }
    }
    // get min
    if !res_sr.is_empty() {
        res_sr.f_sort();
        res_solver.f_sort();
        BestExperimentResult::Success {
            min_total_sr_time: res_sr[0],
            min_total_solver_time: res_solver[0],
            nb_solutions: res_nb_solutions,
        }
    } else if !res_timeout.is_empty() {
        res_timeout.f_sort();
        BestExperimentResult::Timeout {
            max_time_limit: res_timeout[res_timeout.len() - 1],
        }
    } else {
        BestExperimentResult::None
    }
}

fn get_virtual_best(configs: &HashMap<String, Box<ConfigMultiple>>) -> BestExperimentResult {
    let mut v_res_sr = vec![];
    let mut v_res_solver = vec![];
    let mut res_nb_solutions: u64 = 0;
    for v in configs.values() {
        if let BestExperimentResult::Success {
            min_total_sr_time,
            min_total_solver_time,
            nb_solutions,
        } = get_best_time(&*v)
        {
            v_res_sr.push(min_total_sr_time);
            v_res_solver.push(min_total_solver_time);
            if res_nb_solutions == 0 {
                res_nb_solutions = nb_solutions;
            }
        }
    }
    // get min
    if !v_res_sr.is_empty() {
        v_res_sr.f_sort();
        v_res_solver.f_sort();
        BestExperimentResult::Success {
            min_total_sr_time: v_res_sr[0],
            min_total_solver_time: v_res_solver[0],
            nb_solutions: res_nb_solutions,
        }
    } else {
        BestExperimentResult::None
    }
}
pub fn fix_doubts(store: &mut ExperimentStore) {
    for (exp_id, exps) in &mut store.experiments {
        // decide which one to trust
        let mut sol_vote = HashMap::new();
        for configs in exps.configs.values() {
            for solve in &configs.solve_information {
                match solve {
                    SolveInformation::Success { nb_solutions, .. }
                    | SolveInformation::Doubted { nb_solutions, .. } => {
                        let count = sol_vote.entry(*nb_solutions).or_insert(0);
                        *count += 1;
                    }
                    _ => (),
                }
            }
        }
        if sol_vote.len() > 1 {
            let max_vote = sol_vote.values().max().unwrap();
            let max_voted_sols: Vec<u64> = sol_vote
                .iter()
                .filter_map(|(key, val)| if val == max_vote { Some(*key) } else { None })
                .collect();
            if max_voted_sols.len() > 1 {
                let error_str = format!(
                    "There are more than two most voted nb_sols for {} with {:?}. Leaving them as doubted.",
                    exp_id, sol_vote
                );
                // panic!(error_str);
                println!("{}", error_str);
                continue;
            }
            // fix doubted exps and maybe some other success
            for (c_id, configs) in &mut exps.configs {
                for solve in &mut configs.solve_information {
                    match solve {
                        SolveInformation::Success {
                            nb_solutions,
                            memory_limit,
                            time_limit,
                            machine_info,
                            total_sr_time,
                            seed,
                            ..
                        } => {
                            if nb_solutions != &max_voted_sols[0] {
                                println!("Change a previous SUCCESS to CRASHED: details: exp: {} config: {} nb: {}, votes: {:?}",exp_id, c_id, *nb_solutions, sol_vote);
                                *solve = SolveInformation::Crash {
                                    seed: *seed,
                                    memory_limit: *memory_limit,
                                    machine_info: machine_info.clone(),
                                    time_limit: Some(*time_limit),
                                    crash_time: *total_sr_time,
                                };
                            }
                        }
                        SolveInformation::Doubted {
                            total_solver_time,
                            total_sr_time,
                            total_nodes,
                            nb_solutions,
                            seed,
                            memory_limit,
                            time_limit,
                            machine_info,
                            level_info,
                            freq_nb_solutions,
                            ..
                        } => {
                            if nb_solutions == &max_voted_sols[0] {
                                println!("Change a previous DOUBTED to SUCCESS: details: exp: {} config: {} nb: {}, votes: {:?}",exp_id, c_id, *nb_solutions, sol_vote);
                                *solve = SolveInformation::Success {
                                    total_solver_time: *total_solver_time,
                                    total_sr_time: *total_sr_time,
                                    total_nodes: *total_nodes,
                                    nb_solutions: *nb_solutions,
                                    seed: *seed,
                                    memory_limit: *memory_limit,
                                    time_limit: *time_limit,
                                    machine_info: machine_info.clone(),
                                    // I don't like this clone
                                    level_info: level_info.clone(),
                                    freq_nb_solutions: *freq_nb_solutions,
                                };
                            }
                        }
                        _ => (),
                    }
                }
            }
        }
    }
}

impl ExperimentStore {
    pub fn folder_dump(&self, folder_name: &str, compress: bool) {
        let suffix = if compress {
            format!("{}{}", utils::JSON_SUFFIX, utils::ZST_SUFFIX)
        } else {
            utils::JSON_SUFFIX.to_string()
        };
        for (id, e) in &self.experiments {
            let s = serde_json::to_string(&e).unwrap();
            let mut path = std::env::current_dir().unwrap();
            path.push(folder_name);
            if let Some(_m) = fs::metadata(path).ok().filter(|m| m.is_dir()) {
                // nothing
            } else {
                std::fs::create_dir(folder_name).unwrap();
            }
            utils::write_to_file(&format!("{}/{}{}", folder_name, id, suffix), s, compress);
        }
    }

    pub fn from_folder(folder_name: &str, decompress: bool) -> Self {
        let mut exp_store = ExperimentStore {
            experiments: HashMap::new(),
        };
        let mut path = std::env::current_dir().unwrap();
        path.push(folder_name);
        let files = fs::read_dir(path).unwrap();
        for f in files {
            let filename = f.unwrap().path().display().to_string();
            let s = utils::read_file(&filename, decompress);
            let exp_multiple: ExperimentMultiple = serde_json::from_str(&s).unwrap();
            let exp_id = filename
                .split('/')
                .last()
                .unwrap()
                .trim_end_matches(utils::ZST_SUFFIX)
                .trim_end_matches(utils::JSON_SUFFIX)
                .to_string();
            exp_store.experiments.insert(exp_id, Box::new(exp_multiple));
        }
        exp_store
    }
}
